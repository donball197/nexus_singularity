#include <dlfcn.h>
#include <stdio.h>
int main() {
    // Testing the native Termux path we found
    void *h = dlopen("/data/data/com.termux/files/usr/lib/libonnxruntime.so", RTLD_LAZY);
    if (!h) { 
        printf("❌ FAILED: %s\n", dlerror()); 
        return 1; 
    }
    printf("✅ LOADED OK! Node #1 is physically compatible.\n");
    dlclose(h);
    return 0;
}
