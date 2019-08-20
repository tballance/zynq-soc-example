

typedef unsigned int size_t;

#define offsetof(st, m) ((size_t)&(((st *)0)->m))

typedef unsigned int u32;
typedef unsigned char u8;

#define RAM_ADDR 0x1fe00000 //0x5c000000 
struct resource_table {//Just copied from linux/remoteproc.h
    u32 ver;//Must be 1 for remoteproc module!
    u32 num;
    u32 reserved[2];
    u32 offset[1];
} __attribute__((packed));

enum fw_resource_type {
    RSC_CARVEOUT = 0,
    RSC_DEVMEM = 1,
    RSC_TRACE = 2,
    RSC_VDEV = 3,
    RSC_MMU  = 4,
    RSC_LAST = 5,
};

struct fw_rsc_carveout {
    u32 type;//from struct fw_rsc_hdr
    u32 da;
    u32 pa;
    u32 len;
    u32 flags;
    u32 reserved;
    u8 name[32];
} __attribute__((packed));

__attribute__ ((section (".rtable")))
const struct rproc_resource {
    struct resource_table base;
    //u32 offset[4];
    struct fw_rsc_carveout code_cout;
} ti_ipc_remoteproc_ResourceTable = {
 .base = { .ver = 1, .num = 1, .reserved = { 0, 0 },
   .offset = { offsetof(struct rproc_resource, code_cout) },
 },
 .code_cout = {
     .type = RSC_CARVEOUT, .da = RAM_ADDR, .pa = RAM_ADDR, .len = 1<<19,
     .flags=0, .reserved=0, .name="CPU1CODE",
 },
};

