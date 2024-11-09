import config from "./eslint.base.js"
import next from "@next/eslint-plugin-next"

export default config({
  plugins: {
    '@next': next
  }
})
