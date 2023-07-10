/**
 * A simple white noise generator node for demo.
 *
 * @class DummyProcessor
 * @extends AudioWorkletProcessor
 *
 * @link {https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor}
 */
export default class DummyProcessor extends AudioWorkletProcessor {
  // Static getter to define AudioParam objects in this custom processor.
  static get parameterDescriptors() {
    return [
      {
        name: 'gain',
        defaultValue: 0.0,
      },
    ]
  }

  constructor() {
    super()

    this.createdAt_ = currentTime

    this.port.postMessage('DummyProcessor ready! ⚡️')

    this.port.onmessage = (event) => {
      console.log(event.data)
    }
  }

  process(inputs, outputs, parameters) {
    const input = inputs[0]
    const output = outputs[0]
    const len = input.length
    const gain = parameters['gain']

    if (currentTime - this.createdAt_ > 5.0) {
      // do something here
    }

    // Update the data with the new parameters
    for (let channel = 0; channel < len; channel++) {
      const inputChannel = input[channel]
      const outputChannel = output[channel]

      for (let i = 0; i < inputChannel.length; i++) {
        outputChannel[i] = inputChannel[i] * gain[i]
      }
    }

    // Generate white noise
    // output.forEach((channel) => {
    //   for (let i = 0; i < channel.length; i++) {
    //     channel[i] = Math.random() * 2 - 1
    //   }
    // })

    // Return true to keep the worklet working
    return true
  }
}

registerProcessor('dummy', DummyProcessor)
