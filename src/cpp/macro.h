#ifndef RUST_CRYPTOPP_MACRO_H
#define RUST_CRYPTOPP_MACRO_H

#define RCPP_ENUM(rcpp_name, rcpp_path) \
  extern "C" \
  long long enum_ ## rcpp_name () { \
    return rcpp_path; \
  }

#define RCPP_NEW(rcpp_t) \
  extern "C" \
  rcpp_t * new_ ## rcpp_t () { \
    return new rcpp_t (); \
  }

#define RCPP_NEW1(rcpp_name, rcpp_t, rcpp_arg_t) \
  extern "C" \
  rcpp_t * new_ ## rcpp_name ## _ ## rcpp_t (rcpp_arg_t arg) { \
    return new rcpp_t (arg); \
  }

#define RCPP_NEW2(rcpp_name, rcpp_t, rcpp_arg0_t, rcpp_arg1_t) \
  extern "C" \
  rcpp_t * new_ ## rcpp_name ## _ ## rcpp_t (rcpp_arg0_t arg0,\
                                             rcpp_arg1_t arg1) { \
    return new rcpp_t (arg0, arg1); \
  }

#define RCPP_NEW3(rcpp_name, rcpp_t, rcpp_arg0_t, rcpp_arg1_t, rcpp_arg2_t) \
  extern "C" \
  rcpp_t * new_ ## rcpp_name ## _ ## rcpp_t (rcpp_arg0_t arg0, \
                                             rcpp_arg1_t arg1, \
                                             rcpp_arg2_t arg2) { \
    return new rcpp_t (arg0, arg1, arg2); \
  }

#define RCPP_DELETE(rcpp_t) \
  extern "C" \
  void delete_ ## rcpp_t (rcpp_t *ctx) { \
    delete ctx; \
  }

#define RCPP_COPY(rcpp_t) \
  extern "C" \
  rcpp_t * copy_ ## rcpp_t (const rcpp_t *other) { \
    return new rcpp_t (*other); \
  }

#endif /* RUST_CRYPTOPP_MACRO_H */
