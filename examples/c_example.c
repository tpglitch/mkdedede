#include <stdio.h>
#include <string.h>
#include "../include/mkdedede.h"

int main()
{
  printf("--- MKDedede C API Example ---\n\n");

  // 1. Prepare data to encode
  MkddGhostDataC og_data;
  og_data.course = 4;  // Mushroom Bridge
  og_data.kart = 0;    // Red Fire
  og_data.driver1 = 6; // Mario
  og_data.driver2 = 7; // Luigi
  og_data.total_time_ms = 123456;
  og_data.best_lap_ms = 40000;

  printf("Original Data:\n");
  printf("Course: %d, Kart: %d, Drivers: %d | %d\n", og_data.course, og_data.kart, og_data.driver1, og_data.driver2);
  printf("Times: %d (Total), %d (Best Lap)\n\n", og_data.total_time_ms, og_data.best_lap_ms);

  // 2. Encode to string (needs 17 bytes for 16-char password + null terminator)
  char password[17] = {0};
  MkdededeDecodeStatus encode_status = mkdedede_mkdd_encode(&og_data, password);

  if (encode_status != MKDEDEDE_SUCCESS)
  {
    printf("Failed to encode! Status: %d\n", encode_status);
    return 1;
  }

  printf("Encoded Password: %s\n\n", password);

  // 3. Decode back into a struct
  MkddGhostDataC decoded_data;
  MkdededeDecodeStatus decode_status = mkdedede_mkdd_decode(password, &decoded_data);

  if (decode_status == MKDEDEDE_SUCCESS)
  {
    printf("Decoded successfully!\n");
    printf("Course: %d\n", decoded_data.course);
    printf("Kart: %d\n", decoded_data.kart);
    printf("Total Time MS: %d\n", decoded_data.total_time_ms);
  }
  else
  {
    printf("Decode failed with status: %d\n", decode_status);
  }

  return 0;
}
