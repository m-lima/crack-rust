/*
    Original:
    SHA1 OpenCL Optimized kernel
    (c) B. Kerler 2018
    MIT License
*/

union Hash {
  unsigned char bytes[32];
  unsigned int ints[8];
  unsigned long longs[4];
  uint4 vector;
};
typedef union Hash Hash;

#define F1(x, y, z) (bitselect(z,y,x))
#define F0(x, y, z) (bitselect (x, y, ((x) ^ (z))))
#define mod(x, y) ((x)-((x)/(y)*(y)))
#define shr32(x, n) ((x) >> (n))
#define rotl32(a, n) rotate ((a), (n))

#define S0(x) (rotl32 ((x), 25u) ^ rotl32 ((x), 14u) ^ shr32 ((x),  3u))
#define S1(x) (rotl32 ((x), 15u) ^ rotl32 ((x), 13u) ^ shr32 ((x), 10u))
#define S2(x) (rotl32 ((x), 30u) ^ rotl32 ((x), 19u) ^ rotl32 ((x), 10u))
#define S3(x) (rotl32 ((x), 26u) ^ rotl32 ((x), 21u) ^ rotl32 ((x),  7u))

#define SWAP(val) (rotate(((val) & 0x00FF00FF), 24U) | rotate(((val) & 0xFF00FF00), 8U))

__constant uint k_sha256[64] =
{
  0x428a2f98u, 0x71374491u, 0xb5c0fbcfu, 0xe9b5dba5u,
  0x3956c25bu, 0x59f111f1u, 0x923f82a4u, 0xab1c5ed5u,
  0xd807aa98u, 0x12835b01u, 0x243185beu, 0x550c7dc3u,
  0x72be5d74u, 0x80deb1feu, 0x9bdc06a7u, 0xc19bf174u,
  0xe49b69c1u, 0xefbe4786u, 0x0fc19dc6u, 0x240ca1ccu,
  0x2de92c6fu, 0x4a7484aau, 0x5cb0a9dcu, 0x76f988dau,
  0x983e5152u, 0xa831c66du, 0xb00327c8u, 0xbf597fc7u,
  0xc6e00bf3u, 0xd5a79147u, 0x06ca6351u, 0x14292967u,
  0x27b70a85u, 0x2e1b2138u, 0x4d2c6dfcu, 0x53380d13u,
  0x650a7354u, 0x766a0abbu, 0x81c2c92eu, 0x92722c85u,
  0xa2bfe8a1u, 0xa81a664bu, 0xc24b8b70u, 0xc76c51a3u,
  0xd192e819u, 0xd6990624u, 0xf40e3585u, 0x106aa070u,
  0x19a4c116u, 0x1e376c08u, 0x2748774cu, 0x34b0bcb5u,
  0x391c0cb3u, 0x4ed8aa4au, 0x5b9cca4fu, 0x682e6ff3u,
  0x748f82eeu, 0x78a5636fu, 0x84c87814u, 0x8cc70208u,
  0x90befffau, 0xa4506cebu, 0xbef9a3f7u, 0xc67178f2u,
};

#define SHA256_STEP(F0a, F1a, a, b, c, d, e, f, g, h, x, K) \
{                                                           \
  h += K;                                                   \
  h += x;                                                   \
  h += S3(e);                                               \
  h += F1a(e, f, g);                                        \
  d += h;                                                   \
  h += S2(a);                                               \
  h += F0a(a, b, c);                                        \
}

#define SHA256_EXPAND(x, y, z, w) (S1 (x) + y + S0 (z) + w)

static void sha256_process2(const unsigned int * W, unsigned int * digest) {
  unsigned int a = digest[0];
  unsigned int b = digest[1];
  unsigned int c = digest[2];
  unsigned int d = digest[3];
  unsigned int e = digest[4];
  unsigned int f = digest[5];
  unsigned int g = digest[6];
  unsigned int h = digest[7];

  unsigned int w0_t = W[0];
  unsigned int w1_t = W[1];
  unsigned int w2_t = W[2];
  unsigned int w3_t = W[3];
  unsigned int w4_t = W[4];
  unsigned int w5_t = W[5];
  unsigned int w6_t = W[6];
  unsigned int w7_t = W[7];
  unsigned int w8_t = W[8];
  unsigned int w9_t = W[9];
  unsigned int wa_t = W[10];
  unsigned int wb_t = W[11];
  unsigned int wc_t = W[12];
  unsigned int wd_t = W[13];
  unsigned int we_t = W[14];
  unsigned int wf_t = W[15];

#define ROUND_EXPAND(i)                             \
  {                                                 \
    w0_t = SHA256_EXPAND (we_t, w9_t, w1_t, w0_t);  \
    w1_t = SHA256_EXPAND (wf_t, wa_t, w2_t, w1_t);  \
    w2_t = SHA256_EXPAND (w0_t, wb_t, w3_t, w2_t);  \
    w3_t = SHA256_EXPAND (w1_t, wc_t, w4_t, w3_t);  \
    w4_t = SHA256_EXPAND (w2_t, wd_t, w5_t, w4_t);  \
    w5_t = SHA256_EXPAND (w3_t, we_t, w6_t, w5_t);  \
    w6_t = SHA256_EXPAND (w4_t, wf_t, w7_t, w6_t);  \
    w7_t = SHA256_EXPAND (w5_t, w0_t, w8_t, w7_t);  \
    w8_t = SHA256_EXPAND (w6_t, w1_t, w9_t, w8_t);  \
    w9_t = SHA256_EXPAND (w7_t, w2_t, wa_t, w9_t);  \
    wa_t = SHA256_EXPAND (w8_t, w3_t, wb_t, wa_t);  \
    wb_t = SHA256_EXPAND (w9_t, w4_t, wc_t, wb_t);  \
    wc_t = SHA256_EXPAND (wa_t, w5_t, wd_t, wc_t);  \
    wd_t = SHA256_EXPAND (wb_t, w6_t, we_t, wd_t);  \
    we_t = SHA256_EXPAND (wc_t, w7_t, wf_t, we_t);  \
    wf_t = SHA256_EXPAND (wd_t, w8_t, w0_t, wf_t);  \
  }

#define ROUND_STEP(i)                                                     \
  {                                                                       \
    SHA256_STEP (F0, F1, a, b, c, d, e, f, g, h, w0_t, k_sha256[i +  0]); \
    SHA256_STEP (F0, F1, h, a, b, c, d, e, f, g, w1_t, k_sha256[i +  1]); \
    SHA256_STEP (F0, F1, g, h, a, b, c, d, e, f, w2_t, k_sha256[i +  2]); \
    SHA256_STEP (F0, F1, f, g, h, a, b, c, d, e, w3_t, k_sha256[i +  3]); \
    SHA256_STEP (F0, F1, e, f, g, h, a, b, c, d, w4_t, k_sha256[i +  4]); \
    SHA256_STEP (F0, F1, d, e, f, g, h, a, b, c, w5_t, k_sha256[i +  5]); \
    SHA256_STEP (F0, F1, c, d, e, f, g, h, a, b, w6_t, k_sha256[i +  6]); \
    SHA256_STEP (F0, F1, b, c, d, e, f, g, h, a, w7_t, k_sha256[i +  7]); \
    SHA256_STEP (F0, F1, a, b, c, d, e, f, g, h, w8_t, k_sha256[i +  8]); \
    SHA256_STEP (F0, F1, h, a, b, c, d, e, f, g, w9_t, k_sha256[i +  9]); \
    SHA256_STEP (F0, F1, g, h, a, b, c, d, e, f, wa_t, k_sha256[i + 10]); \
    SHA256_STEP (F0, F1, f, g, h, a, b, c, d, e, wb_t, k_sha256[i + 11]); \
    SHA256_STEP (F0, F1, e, f, g, h, a, b, c, d, wc_t, k_sha256[i + 12]); \
    SHA256_STEP (F0, F1, d, e, f, g, h, a, b, c, wd_t, k_sha256[i + 13]); \
    SHA256_STEP (F0, F1, c, d, e, f, g, h, a, b, we_t, k_sha256[i + 14]); \
    SHA256_STEP (F0, F1, b, c, d, e, f, g, h, a, wf_t, k_sha256[i + 15]); \
  }

  ROUND_STEP (0);

  ROUND_EXPAND();
  ROUND_STEP(16);

  ROUND_EXPAND();
  ROUND_STEP(32);

  ROUND_EXPAND();
  ROUND_STEP(48);

  digest[0] += a;
  digest[1] += b;
  digest[2] += c;
  digest[3] += d;
  digest[4] += e;
  digest[5] += f;
  digest[6] += g;
  digest[7] += h;
}

/* The main hashing function */
static void sha256(unsigned int * hash, const unsigned int * input) {
  int input_len = CONST_LENGTH / 4;
  if (mod(CONST_LENGTH, 4)) {
    input_len++;
  }

  unsigned int W[0x10] = {0};
  int loops = input_len;
  int current_loop = 0;
  unsigned int State[8] = {0};
  State[0] = 0x6a09e667;
  State[1] = 0xbb67ae85;
  State[2] = 0x3c6ef372;
  State[3] = 0xa54ff53a;
  State[4] = 0x510e527f;
  State[5] = 0x9b05688c;
  State[6] = 0x1f83d9ab;
  State[7] = 0x5be0cd19;

  while (loops > 0) {
    W[0x0] = 0x0;
    W[0x1] = 0x0;
    W[0x2] = 0x0;
    W[0x3] = 0x0;
    W[0x4] = 0x0;
    W[0x5] = 0x0;
    W[0x6] = 0x0;
    W[0x7] = 0x0;
    W[0x8] = 0x0;
    W[0x9] = 0x0;
    W[0xA] = 0x0;
    W[0xB] = 0x0;
    W[0xC] = 0x0;
    W[0xD] = 0x0;
    W[0xE] = 0x0;
    W[0xF] = 0x0;

    for (int m = 0; loops != 0 && m < 16; m++) {
      W[m] ^= SWAP(input[m + (current_loop * 16)]);
      loops--;
    }

    if (loops == 0 && mod(CONST_LENGTH, 64) != 0) {
      unsigned int padding = 0x80 << (((CONST_LENGTH + 4) - ((CONST_LENGTH + 4) / 4 * 4)) * 8);
      int v = mod(CONST_LENGTH, 64);
      W[v / 4] |= SWAP(padding);
      if ((CONST_LENGTH & 0x3B) != 0x3B) {
        /* Let's add length */
        W[0x0F] = CONST_LENGTH * 8;
      }
    }

    sha256_process2(W, State);
    current_loop++;
  }

  if (mod(input_len, 16) == 0) {
    W[0x0] = 0x0;
    W[0x1] = 0x0;
    W[0x2] = 0x0;
    W[0x3] = 0x0;
    W[0x4] = 0x0;
    W[0x5] = 0x0;
    W[0x6] = 0x0;
    W[0x7] = 0x0;
    W[0x8] = 0x0;
    W[0x9] = 0x0;
    W[0xA] = 0x0;
    W[0xB] = 0x0;
    W[0xC] = 0x0;
    W[0xD] = 0x0;
    W[0xE] = 0x0;
    W[0xF] = 0x0;

    if ((CONST_LENGTH & 0x3B) != 0x3B) {
      unsigned int padding = 0x80 << (((CONST_LENGTH + 4) - ((CONST_LENGTH + 4) / 4 * 4)) * 8);
      W[0] |= SWAP(padding);
    }

    /* Let's add length */
    W[0x0F] = CONST_LENGTH * 8;

    sha256_process2(W, State);
  }

  hash[0] = SWAP(State[0]);
  hash[1] = SWAP(State[1]);
  hash[2] = SWAP(State[2]);
  hash[3] = SWAP(State[3]);
  hash[4] = SWAP(State[4]);
  hash[5] = SWAP(State[5]);
  hash[6] = SWAP(State[6]);
  hash[7] = SWAP(State[7]);
  return;
}

#undef F0
#undef F1
#undef S0
#undef S1
#undef S2
#undef S3

#undef mod
#undef shr32
#undef rotl32

//______________________________________________________________________________
// Find the hash for a limited number of targets
//
// Defines:
// CONST_BEGIN {:d} # The index of where the variable part begins
// CONST_END {:d} # The index past of where the variable part ends
// CONST_BASE64_BEGIN {:d} # The index where to base64 encode from (salt length)
// CONST_LENGTH {:d} # The length of the payload (salt + value)
// CONST_LENGTH_ON_CPU {:d} # Decimal places the iterations are substituting
// CONST_TARGET_COUNT {:d} # The number of items in the targets array
//
// targets: Target hashes
// output: Matched values
//______________________________________________________________________________
__kernel void crack(constant Hash * targets,
    global unsigned int * output,
    private const unsigned int prefix) {
  unsigned int index = get_global_id(0);

  // Buffer for the hash
  Hash hash;

  // Zero initialize
  Value value = {};

#ifdef CONST_LENGTH_ON_CPU
  prepare(index, prefix, &value);
#else
  prepare(index, &value);
#endif

  // %%PREFIX%%

#ifdef CONST_BASE64_BEGIN
  // %%XOR%%
  to_base64(&value);
#endif

  // Inject size
  value.longs[7] = CONST_LENGTH << 3;

  // Inject padding
  value.bytes[CONST_LENGTH] = 0x80;

  // Actually cracking
  sha256(hash.ints, value.ints);

#if CONST_TARGET_COUNT < 32
#pragma unroll
  for (unsigned int i = 0; i < CONST_TARGET_COUNT; i++) {
    if (hash.longs[3] == targets[i].longs[3]
        && hash.longs[2] == targets[i].longs[2]
        && hash.longs[1] == targets[i].longs[1]
        && hash.longs[0] == targets[i].longs[0]) {
      output[i << 1] = (unsigned int)(index & 0xFFFFFF);
      output[(i << 1) + 1] = (unsigned int)(prefix & 0xFFFFFF);
      return;
    }
  }
#else
  unsigned int i = 0;
  while (i < CONST_TARGET_COUNT) {
    if (hash.longs[3] == targets[i].longs[3]) {
      if (hash.longs[2] == targets[i].longs[2]) {
        if (hash.longs[1] == targets[i].longs[1]) {
          if (hash.longs[0] == targets[i].longs[0]) {
            output[i << 1] = (unsigned int)(index & 0xFFFFFF);
            output[(i << 1) + 1] = (unsigned int)(prefix & 0xFFFFFF);
            return;
          } else {
            i = hash.longs[0] < targets[i].longs[0]
              ? (i << 1) + 1
              : (i << 1) + 2;
          }
        } else {
          i = hash.longs[1] < targets[i].longs[1]
            ? (i << 1) + 1
            : (i << 1) + 2;
        }
      } else {
        i = hash.longs[2] < targets[i].longs[2]
          ? (i << 1) + 1
          : (i << 1) + 2;
      }
    } else {
      i = hash.longs[3] < targets[i].longs[3]
        ? (i << 1) + 1
        : (i << 1) + 2;
    }
  }
#endif //#if CONST_TARGET_COUNT
}
