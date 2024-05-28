import {useToast} from 'vue-toastification'

const mxToast = {
  data() {
    return {
      toast: useToast()
    }
  },

  methods: {
    cleanErrorMessage(errorMessage) {
      const startPattern = "message index: 0: ";
      const endPattern = ": execute wasm contract failed";

      const startIndex = errorMessage.indexOf(startPattern);
      const endIndex = errorMessage.indexOf(endPattern);

      if (startIndex !== -1 && endIndex !== -1) {
        return errorMessage.substring(startIndex + startPattern.length, endIndex);
      }

      // Return the original message if the patterns aren't found
      return errorMessage;
    }
  }
}

export default mxToast
