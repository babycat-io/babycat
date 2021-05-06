use crate::{Array, ArrayBase, DataMut, Dimension, IntoNdProducer, NdProducer, Zip};
use crate::AssignElem;

use crate::parallel::prelude::*;
use crate::parallel::par::ParallelSplits;
use super::send_producer::SendProducer;

use crate::partial::Partial;

/// # Parallel methods
///
/// These methods require crate feature `rayon`.
impl<A, S, D> ArrayBase<S, D>
where
    S: DataMut<Elem = A>,
    D: Dimension,
    A: Send + Sync,
{
    /// Parallel version of `map_inplace`.
    ///
    /// Modify the array in place by calling `f` by mutable reference on each element.
    ///
    /// Elements are visited in arbitrary order.
    pub fn par_map_inplace<F>(&mut self, f: F)
    where
        F: Fn(&mut A) + Sync + Send,
    {
        self.view_mut().into_par_iter().for_each(f)
    }

    /// Parallel version of `mapv_inplace`.
    ///
    /// Modify the array in place by calling `f` by **v**alue on each element.
    /// The array is updated with the new values.
    ///
    /// Elements are visited in arbitrary order.
    pub fn par_mapv_inplace<F>(&mut self, f: F)
    where
        F: Fn(A) -> A + Sync + Send,
        A: Clone,
    {
        self.view_mut()
            .into_par_iter()
            .for_each(move |x| *x = f(x.clone()))
    }
}

// Zip

const COLLECT_MAX_SPLITS: usize = 10;

macro_rules! zip_impl {
    ($([$notlast:ident $($p:ident)*],)+) => {
        $(
        #[allow(non_snake_case)]
        impl<D, $($p),*> Zip<($($p,)*), D>
            where $($p::Item : Send , )*
                  $($p : Send , )*
                  D: Dimension,
                  $($p: NdProducer<Dim=D> ,)*
        {
            /// The `par_apply` method for `Zip`.
            ///
            /// This is a shorthand for using `.into_par_iter().for_each()` on
            /// `Zip`.
            ///
            /// Requires crate feature `rayon`.
            pub fn par_apply<F>(self, function: F)
                where F: Fn($($p::Item),*) + Sync + Send
            {
                self.into_par_iter().for_each(move |($($p,)*)| function($($p),*))
            }

            expand_if!(@bool [$notlast]

            /// Apply and collect the results into a new array, which has the same size as the
            /// inputs.
            ///
            /// If all inputs are c- or f-order respectively, that is preserved in the output.
            pub fn par_apply_collect<R>(self, f: impl Fn($($p::Item,)* ) -> R + Sync + Send)
                -> Array<R, D>
                where R: Send
            {
                let mut output = self.uninitalized_for_current_layout::<R>();
                let total_len = output.len();

                // Create a parallel iterator that produces chunks of the zip with the output
                // array.  It's crucial that both parts split in the same way, and in a way
                // so that the chunks of the output are still contig.
                //
                // Use a raw view so that we can alias the output data here and in the partial
                // result.
                let splits = unsafe {
                    ParallelSplits {
                        iter: self.and(SendProducer::new(output.raw_view_mut().cast::<R>())),
                        // Keep it from splitting the Zip down too small
                        max_splits: COLLECT_MAX_SPLITS,
                    }
                };

                let collect_result = splits.map(move |zip| {
                    // Apply the mapping function on this chunk of the zip
                    // Create a partial result for the contiguous slice of data being written to
                    unsafe {
                        zip.collect_with_partial(&f)
                    }
                })
                .reduce(Partial::stub, Partial::try_merge);

                if std::mem::needs_drop::<R>() {
                    debug_assert_eq!(total_len, collect_result.len,
                        "collect len is not correct, expected {}", total_len);
                    assert!(collect_result.len == total_len,
                        "Collect: Expected number of writes not completed");
                }

                // Here the collect result is complete, and we release its ownership and transfer
                // it to the output array.
                collect_result.release_ownership();
                unsafe {
                    output.assume_init()
                }
            }

            /// Apply and assign the results into the producer `into`, which should have the same
            /// size as the other inputs.
            ///
            /// The producer should have assignable items as dictated by the `AssignElem` trait,
            /// for example `&mut R`.
            pub fn par_apply_assign_into<R, Q>(self, into: Q, f: impl Fn($($p::Item,)* ) -> R + Sync + Send)
                where Q: IntoNdProducer<Dim=D>,
                      Q::Item: AssignElem<R> + Send,
                      Q::Output: Send,
            {
                self.and(into)
                    .par_apply(move |$($p, )* output_| {
                        output_.assign_elem(f($($p ),*));
                    });
            }
            );
        }
        )+
    }
}

zip_impl! {
    [true P1],
    [true P1 P2],
    [true P1 P2 P3],
    [true P1 P2 P3 P4],
    [true P1 P2 P3 P4 P5],
    [false P1 P2 P3 P4 P5 P6],
}
