#include <cryptopp/cryptlib.h>

using namespace CryptoPP;

extern "C"
void HashTransformation_Update(HashTransformation *ctx,
                               const unsigned char *input,
                               size_t length) {
  ctx->Update(input, length);
}

extern "C"
void HashTransformation_Final(HashTransformation *ctx,
                              unsigned char *digest) {
  ctx->Final(digest);
}
