// Byte per audio sample. (32 bit float)
export const BYTES_PER_SAMPLE = Float32Array.BYTES_PER_ELEMENT

// Basic byte unit of WASM heap. (16 bit = 2 bytes)
export const BYTES_PER_UNIT = Uint16Array.BYTES_PER_ELEMENT

// The max audio channel on Chrome is 32.
export const MAX_CHANNEL_COUNT = 32

// WebAudio's render quantum size.
export const RENDER_QUANTUM_FRAMES = 128
