import config from "./eslint.base"

export default [
  ...config,
  {
    extends: [
      "plugin:@next/next/recommende"
    ]
  }
]
