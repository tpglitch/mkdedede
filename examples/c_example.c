#include <stdio.h>
#include <string.h>
#include "../include/mkdedede.h"

int main()
{
  printf("--- MKDedede C API Example ---\n\n");

  // 1. Prepare data to encode
  MkddGhostDataC og_data;
  og_data.course = MKDD_COURSE_MUSHROOMBRIDGE;
  og_data.kart = MKDD_KART_REDFIRE;
  og_data.driver1 = MKDD_CHARACTER_MARIO;
  og_data.driver2 = MKDD_CHARACTER_LUIGI;
  og_data.total_time_ms = 123456;
  og_data.best_lap_ms = 40000;

  printf("Original Data:\n");
  printf("Course: %s\n", mkdedede_mkdd_course_name(og_data.course));
  printf("Kart: %s\n", mkdedede_mkdd_kart_name(og_data.kart));
  printf("Drivers: %s | %s\n", mkdedede_mkdd_character_name(og_data.driver1), mkdedede_mkdd_character_name(og_data.driver2));

  char total_time[16] = {0};
  char best_lap[16] = {0};
  mkdedede_mkdd_format_time(og_data.total_time_ms, total_time);
  mkdedede_mkdd_format_time(og_data.best_lap_ms, best_lap);
  printf("Times: %s (Total), %s (Best Lap)\n", total_time, best_lap);
  printf("Laps: %u\n\n", mkdedede_mkdd_course_laps(og_data.course));

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
    printf("Course: %s\n", mkdedede_mkdd_course_name(decoded_data.course));
    printf("Kart: %s\n", mkdedede_mkdd_kart_name(decoded_data.kart));

    char decoded_total[16] = {0};
    char decoded_best[16] = {0};
    mkdedede_mkdd_format_time(decoded_data.total_time_ms, decoded_total);
    mkdedede_mkdd_format_time(decoded_data.best_lap_ms, decoded_best);
    printf("Total Time: %s\n", decoded_total);
    printf("Best Lap: %s\n", decoded_best);
  }
  else
  {
    printf("Decode failed with status: %d\n", decode_status);
  }

  return 0;
}
