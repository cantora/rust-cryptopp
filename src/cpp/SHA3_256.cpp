#include <cryptopp/sha3.h>

using namespace CryptoPP;

extern "C"
SHA3_256 *SHA3_256_new() {
  return new SHA3_256();
}

extern "C"
void SHA3_256_delete(SHA3_256 *ctx) {
  delete ctx;
}
