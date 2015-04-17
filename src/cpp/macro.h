#ifndef RUST_CRYPTOPP_MACRO_H
#define RUST_CRYPTOPP_MACRO_H

#define RCPP_NEW(rcpp_typ) \
  extern "C" rcpp_typ * new_ ## rcpp_typ () { \
    return new rcpp_typ (); \
  }

#define RCPP_DELETE(rcpp_typ) \
  extern "C" void delete_ ## rcpp_typ (rcpp_typ *ctx) { \
    delete ctx; \
  }

#endif /* RUST_CRYPTOPP_MACRO_H */
