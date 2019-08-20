#ifndef __STDDEF_H
#define __STDDEF_H

#ifdef __cplusplus
extern "C" {
#endif

#ifdef __cplusplus
#define NULL 0
#else
#define NULL ((void *)0)
#endif

#ifdef __GNUC__
typedef __SIZE_TYPE__ size_t;
typedef __PTRDIFF_TYPE__ ptrdiff_t;
#else
typedef unsigned long size_t;
typedef long ptrdiff_t;
#endif

#define offsetof(type, member) __builtin_offsetof(type, member)

#ifdef __cplusplus
}
#endif

#endif /* __STDDEF_H */
