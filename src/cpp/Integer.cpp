#include "macro.h"

#include <cryptopp/integer.h>

using namespace CryptoPP;

RCPP_ENUM(Integer_UNSIGNED, Integer::UNSIGNED)
RCPP_ENUM(Integer_SIGNED,   Integer::SIGNED)

RCPP_NEW(Integer)
RCPP_DELETE(Integer)
RCPP_COPY(Integer)

RCPP_NEW1(from_long, Integer, signed long)
