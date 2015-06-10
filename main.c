#include <dlfcn.h>
#include <stdio.h>
#include <assert.h>

typedef union{
  void* not_a_function;
  void* (*fn)(int,char**);
} Trick;


/*More error codes will be added as a features.*/
typedef enum{
  DIDNT_WORK=1,
  MOSTLY_WORKING=127,
  DID_WORK=255
}FRMWK_ERRORS;

int main(int argc, char** argv){
  static void* errref = NULL;


  Trick trick;
  void* plugin = dlopen(argv[1],RTLD_NOW);
  trick.not_a_function=dlsym(plugin,"main");
  FRMWK_ERRORS err = (FRMWK_ERRORS) trick.fn(argc-1,argv+1);

  if (err != DIDNT_WORK && err != MOSTLY_WORKING && err != DID_WORK){
    assert(0==1);
    printf("DID NOT USE ESTABLISHED ERROR FORMAT: ");
    printf("%d\n",((int*)(errref))[err]);
  }
  /*optimized return value.*/
  return ~(1-DIDNT_WORK);
}
