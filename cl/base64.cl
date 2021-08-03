#ifdef CONST_BASE64_BEGIN
__constant char base64_map[64] = {
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/'
};

inline void to_base64(Value * value) {
  unsigned char bytes[CONST_END - CONST_BASE64_BEGIN];
  unsigned char index = CONST_BASE64_BEGIN;

#pragma unroll
  for (unsigned char i = CONST_BASE64_BEGIN; i < CONST_END; i++) {
    bytes[i - CONST_BASE64_BEGIN] = value->bytes[i];
  }

#pragma unroll
  for (unsigned char i = 0; i < CONST_END - CONST_BASE64_BEGIN;) {
    unsigned char x, y, z;

    x = bytes[i++];

    // xxxxxxxxyyyyyyyyzzzzzzzz
    // aaaaaabbbbbbccccccdddddd

    // xxxxxxxx >> 2
    // xxxxxx
    // aaaaaa
    value->bytes[index++] = base64_map[x >> 2];

    if (i < CONST_END - CONST_BASE64_BEGIN) {
      y = bytes[i++];

      // xxxxxxxx & 0b11
      //       xx << 4
      //   xx____
      // yyyyyyyy >> 4
      // yyyy
      //   xx____ | yyyy
      // xxyyyy
      // bbbbbb
      value->bytes[index++] = base64_map[((x & 0b11) << 4) | (y >> 4)];

      if (i < CONST_END - CONST_BASE64_BEGIN) {
        z = bytes[i++];

        // yyyyyyyy & 0b1111
        //     yyyy << 2
        //   yyyy__
        // zzzzzzzz >> 6
        // zz
        //   yyyy__ | zz
        // yyyyzz
        // cccccc
        value->bytes[index++] = base64_map[((y & 0b1111) << 2) | (z >> 6)];

        // zzzzzzzz & 0b111111
        // zzzzzz
        // dddddd
        value->bytes[index++] = base64_map[z & 0b111111];
      } else {
        value->bytes[index++] = base64_map[(y & 0b1111) << 2];
        value->bytes[index++] = '=';
      }
    } else {
      value->bytes[index++] = base64_map[(x & 0b11) << 4];
      value->bytes[index++] = '=';
      value->bytes[index++] = '=';
    }
  }
}
#endif
