union Value {
  unsigned char bytes[64];
  unsigned int ints[16];
  unsigned long longs[8];
};
typedef union Value Value;

/*
 * The skeleton is composed of: [SUFFIX + variable + LENGTH]
 * The size of the skeleton is a multiple of 512 bits (64 bytes)
 * The SUFFIX is arbitrary
 * The LENGTH is the length of (SUFFIX + variable) in a 64 bit format
 * If LENGTH happens to be larger than 64 bits, only the lower 64 bits
 * are considered
 */
#ifdef CONST_LENGTH_ON_CPU
inline void prepare(unsigned int value,
    unsigned int iteration,
    Value * skeleton) {

  unsigned int next;

  // Filling the "variable" part of the skeleton
#pragma unroll
  for (char index = CONST_END - 1;
      index >= CONST_BEGIN + CONST_LENGTH_ON_CPU;
      index--)
  {
    // Next decimal place
    next = value / 10;

    // Convert numbers to char
    skeleton->bytes[index] = (value - next * 10) + 48;

    // Move one decimal place
    value = next;
  }

  // Filling the iteration part of the skeleton
#pragma unroll
  for (char index = CONST_BEGIN + CONST_LENGTH_ON_CPU - 1;
      index >= CONST_BEGIN;
      index--)
  {
    // Next decimal place
    next = iteration / 10;

    // Convert numbers to char
    skeleton->bytes[index] = (iteration - next * 10) + 48;

    // Move one decimal place
    iteration = next;
  }
}
#else
inline void prepare(unsigned int value, Value * skeleton) {

  unsigned int next;

  // Filling the "variable" part of the skeleton
#pragma unroll
  for (char index = CONST_END - 1; index >= CONST_BEGIN; index--) {
    // Next decimal place
    next = value / 10;

    // Convert numbers to char
    skeleton->bytes[index] = (value - next * 10) + 48;

    // Move one decimal place
    value = next;
  }
}
#endif
