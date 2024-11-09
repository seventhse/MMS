// eslint.config.js
import antfu from '@antfu/eslint-config'

export default (configs) => antfu({
  react: true,
  ...configs
})
