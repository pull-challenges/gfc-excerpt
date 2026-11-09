#include <gfc/gfc.h>
#include <stdio.h>
#include <stdlib.h>

void print_decrypted(uint64_t input) {
  GFC *gfc = gfc_init(100, 6, 42);

  uint64_t enc = gfc_encrypt(gfc, input);
  uint64_t dec = gfc_decrypt(gfc, enc);

  printf("enc = %lu\n", enc);
  printf("dec = %lu\n", dec);

  gfc_destroy(gfc);
}

int main(int argc, char** argv) {
  if (argc < 2) {
    fprintf(stderr, "Usage: %s <number>\n", argv[0]);
    return 1;
  }

  char* endptr;
  uint64_t input_num = strtoull(argv[1], &endptr, 0);

  print_decrypted(input_num);
  return 0;
}
