import { Waveform } from "babycat";

function babycatDecode(arrayBuffer) {
  const arr = new Uint8Array(arrayBuffer);
  const waveform = Waveform.fromEncodedArray(arr, {});
  console.log(
    "Decoded",
    waveform.numFrames(),
    "frames with",
    waveform.numChannels(),
    "at",
    waveform.frameRateHz(),
    "hz"
  );
}

function handleFileUpload() {
  this.files[0].arrayBuffer().then((arrayBuffer) => babycatDecode(arrayBuffer));
}

function createFileDialog() {
  const fileUploader = document.createElement("input");
  fileUploader.type = "file";
  fileUploader.id = "fileUploader";
  fileUploader.addEventListener("change", handleFileUpload, false);

  return fileUploader;
}

document.body.appendChild(createFileDialog());
