
export class LowpassFilter extends AudioWorkletProcessor {
  static get parameterDescriptors() {
    return [
      {
        name: 'frequency',
        defaultValue: 100,
      },
    ]
  }

  constructor() {
    super()

    this.lastOut_ = 0
  }

  process(inputs, outputs, parameters) {
    const input = inputs[0]
    const output = outputs[0]
    const frequency = parameters.frequency
    const len = output.length
    let coeff = 0

    for (let channel = 0; channel < len; channel++) {
      const inputChannel = input[channel]
      const outputChannel = output[channel]
      coeff = 2 * Math.PI * frequency[0] / sampleRate

      for (let i = 0; i < outputChannel.length; i++) {
        outputChannel[i] = inputChannel[i] * coeff + (1 - coeff) * this.lastOut_
        this.lastOut_ = outputChannel[i]
      }
    }
  
    return true
  }
}

registerProcessor('lowpass-filter', LowpassFilter)