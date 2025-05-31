#ifndef _DEF_WINBOOL_
#define _DEF_WINBOOL_
typedef int WINBOOL;
#pragma push_macro("BOOL")
#undef BOOL
#if !defined(__OBJC__) && !defined(__OBJC_BOOL) && !defined(__objc_INCLUDE_GNU) && !defined(_NO_BOOL_TYPEDEF)
  typedef int BOOL;
#endif
#define BOOL WINBOOL
typedef BOOL *PBOOL;
typedef BOOL *LPBOOL;
#pragma pop_macro("BOOL")
#endif /* _DEF_WINBOOL_ */

#include <d3drm.h>