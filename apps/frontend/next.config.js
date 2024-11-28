export default {
  async headers() {
    return [
      {
        source: '/',
        headers: [
          {
            key: 'x-middleware-cache',
            value: 'no-cache',
          },
        ],
      },
    ]
  },
}
