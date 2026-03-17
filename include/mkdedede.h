#ifndef MKDEDEDE_H
#define MKDEDEDE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

  typedef enum MkddCourseC
  {
    MKDD_COURSE_LUIGICIRCUIT = 0,
    MKDD_COURSE_PEACHBEACH,
    MKDD_COURSE_BABYPARK,
    MKDD_COURSE_DRYDRYDESERT,
    MKDD_COURSE_MUSHROOMBRIDGE,
    MKDD_COURSE_MARIOCIRCUIT,
    MKDD_COURSE_DAISYCRUISER,
    MKDD_COURSE_WALUIGISTADIUM,
    MKDD_COURSE_SHERBETLAND,
    MKDD_COURSE_MUSHROOMCITY,
    MKDD_COURSE_YOSHICIRCUIT,
    MKDD_COURSE_DKMOUNTAIN,
    MKDD_COURSE_WARIOCOLOSSEUM,
    MKDD_COURSE_DINODINOJUNGLE,
    MKDD_COURSE_BOWSERSCASTLE,
    MKDD_COURSE_RAINBOWROAD,
  } MkddCourseC;

  typedef enum MkddCharacterC
  {
    MKDD_CHARACTER_BABYMARIO = 0,
    MKDD_CHARACTER_BABYLUIGI,
    MKDD_CHARACTER_PARATROOPA,
    MKDD_CHARACTER_KOOPA,
    MKDD_CHARACTER_PEACH,
    MKDD_CHARACTER_DAISY,
    MKDD_CHARACTER_MARIO,
    MKDD_CHARACTER_LUIGI,
    MKDD_CHARACTER_WARIO,
    MKDD_CHARACTER_WALUIGI,
    MKDD_CHARACTER_YOSHI,
    MKDD_CHARACTER_BIRDO,
    MKDD_CHARACTER_DONKEYKONG,
    MKDD_CHARACTER_DIDDYKONG,
    MKDD_CHARACTER_BOWSER,
    MKDD_CHARACTER_BOWSERJR,
    MKDD_CHARACTER_TOAD,
    MKDD_CHARACTER_TOADETTE,
    MKDD_CHARACTER_KINGBOO,
    MKDD_CHARACTER_PETEYPIRANHA,
  } MkddCharacterC;

  typedef enum MkddKartC
  {
    MKDD_KART_REDFIRE = 0,
    MKDD_KART_DKJUMBO,
    MKDD_KART_TURBOYOSHI,
    MKDD_KART_KOOPADASHER,
    MKDD_KART_HEARTCOACH,
    MKDD_KART_GOOGOOBUGGY,
    MKDD_KART_WARIOCAR,
    MKDD_KART_KOOPAKING,
    MKDD_KART_GREENFIRE,
    MKDD_KART_BARRELTRAIN,
    MKDD_KART_TURBOBIRDO,
    MKDD_KART_PARAWING,
    MKDD_KART_BLOOMCOACH,
    MKDD_KART_RATTLEBUGGY,
    MKDD_KART_WALUIGIRACER,
    MKDD_KART_BULLETBLASTER,
    MKDD_KART_TOADKART,
    MKDD_KART_TOADETTEKART,
    MKDD_KART_BOOPIPES,
    MKDD_KART_PIRANHAPIPES,
    MKDD_KART_PARADEKART,
  } MkddKartC;

  typedef enum MkdsCourseC
  {
    MKDS_COURSE_FIGURE8CIRCUIT = 0,
    MKDS_COURSE_YOSHIFALLS,
    MKDS_COURSE_CHEEPCHEEPBEACH,
    MKDS_COURSE_LUIGISMANSION,
    MKDS_COURSE_DESERTHILLS,
    MKDS_COURSE_DELFINOSQUARE,
    MKDS_COURSE_WALUIGIPINBALL,
    MKDS_COURSE_SHROOMRIDGE,
    MKDS_COURSE_DKPASS,
    MKDS_COURSE_TICKTOCKCLOCK,
    MKDS_COURSE_MARIOCIRCUIT,
    MKDS_COURSE_AIRSHIPFORTRESS,
    MKDS_COURSE_WARIOSTADIUM,
    MKDS_COURSE_PEACHGARDENS,
    MKDS_COURSE_BOWSERCASTLE,
    MKDS_COURSE_RAINBOWROAD,
    MKDS_COURSE_SNESMARIOCIRCUIT1,
    MKDS_COURSE_N64MOOMOOFARM,
    MKDS_COURSE_GBAPEACHCIRCUIT,
    MKDS_COURSE_GCNLUIGICIRCUIT,
    MKDS_COURSE_SNESDONUTPLAINS1,
    MKDS_COURSE_N64FRAPPESNOWLAND,
    MKDS_COURSE_GBABOWSERCASTLE2,
    MKDS_COURSE_GCNBABYPARK,
    MKDS_COURSE_SNESKOOPABEACH2,
    MKDS_COURSE_N64CHOCOMOUNTAIN,
    MKDS_COURSE_GBALUIGICIRCUIT,
    MKDS_COURSE_GCNMUSHROOMBRIDGE,
    MKDS_COURSE_SNESCHOCOISLAND2,
    MKDS_COURSE_N64BANSHEEBOARDWALK,
    MKDS_COURSE_GBASKYGARDEN,
    MKDS_COURSE_GCNYOSHICIRCUIT,
  } MkdsCourseC;

  typedef enum MkdsCharacterC
  {
    MKDS_CHARACTER_MARIO = 0,
    MKDS_CHARACTER_DONKEYKONG,
    MKDS_CHARACTER_TOAD,
    MKDS_CHARACTER_BOWSER,
    MKDS_CHARACTER_PEACH,
    MKDS_CHARACTER_WARIO,
    MKDS_CHARACTER_YOSHI,
    MKDS_CHARACTER_LUIGI,
    MKDS_CHARACTER_DRYBONES,
    MKDS_CHARACTER_DAISY,
    MKDS_CHARACTER_WALUIGI,
    MKDS_CHARACTER_ROB,
    MKDS_CHARACTER_SHYGUY,
  } MkdsCharacterC;

  typedef enum MkdsKartC
  {
    MKDS_KART_STANDARDMR = 0,
    MKDS_KART_SHOOTINGSTAR,
    MKDS_KART_BDASHER,
    MKDS_KART_STANDARDDK,
    MKDS_KART_WILDLIFE,
    MKDS_KART_RAMBIRIDER,
    MKDS_KART_STANDARDTD,
    MKDS_KART_MUSHMELLOW,
    MKDS_KART_FOURWHEELCRADLE,
    MKDS_KART_STANDARDBW,
    MKDS_KART_HURRICANE,
    MKDS_KART_TYRANT,
    MKDS_KART_STANDARDPC,
    MKDS_KART_LIGHTTRIPPER,
    MKDS_KART_ROYALE,
    MKDS_KART_STANDARDWR,
    MKDS_KART_BRUTE,
    MKDS_KART_DRAGONFLY,
    MKDS_KART_STANDARDYS,
    MKDS_KART_EGG1,
    MKDS_KART_CUCUMBER,
    MKDS_KART_STANDARDLG,
    MKDS_KART_POLTERGUST4000,
    MKDS_KART_STREAMLINER,
    MKDS_KART_STANDARDDB,
    MKDS_KART_DRYBOMBER,
    MKDS_KART_BANISHER,
    MKDS_KART_STANDARDDS,
    MKDS_KART_LIGHTDANCER,
    MKDS_KART_POWERFLOWER,
    MKDS_KART_STANDARDWL,
    MKDS_KART_GOLDMANTIS,
    MKDS_KART_ZIPPER,
    MKDS_KART_STANDARDRB,
    MKDS_KART_ROBBLS,
    MKDS_KART_ROBLGS,
    MKDS_KART_STANDARDSG,
  } MkdsKartC;

  typedef struct MkddGhostDataC
  {
    uint8_t course;
    uint8_t kart;
    uint8_t driver1;
    uint8_t driver2;
    uint32_t total_time_ms;
    uint32_t best_lap_ms;
  } MkddGhostDataC;

  typedef struct DsGhostDataC
  {
    uint8_t course;
    uint8_t character;
    uint8_t kart;
    uint32_t time_ms;
    uint16_t player_name[2];
  } DsGhostDataC;

  typedef enum MkdededeDecodeStatus
  {
    MKDEDEDE_SUCCESS = 0,
    MKDEDEDE_WRONG_LENGTH = 1,
    MKDEDEDE_INVALID_CHARACTER = 2,
    MKDEDEDE_INVALID_CHECKSUM = 3,
    MKDEDEDE_INVALID_COURSE = 4,
    MKDEDEDE_INVALID_KART = 5,
    MKDEDEDE_INVALID_DRIVER = 6,
    MKDEDEDE_NULL_POINTER = 7,
    MKDEDEDE_UTF8_ERROR = 8,
    MKDEDEDE_SAME_DRIVERS = 9,
    MKDEDEDE_INVALID_TIMES = 10,
  } MkdededeDecodeStatus;

  /**
   * Decodes a Mario Kart: Double Dash!! 16-character password into ghost data
   * @param password Null-terminated string containing the password to decode
   * @param out_data Pointer to the struct where decoded track info will be written
   * @return MKDEDEDE_SUCCESS on success, or an error status on failure
   */
  MkdededeDecodeStatus mkdedede_mkdd_decode(const char *password, MkddGhostDataC *out_data);

  /**
   * Decodes a Mario Kart DS 16-character password into ghost data
   * @param password Null-terminated string containing the password to decode
   * @param out_data Pointer to the struct where decoded track info will be written
   * @return MKDEDEDE_SUCCESS on success, or an error status on failure
   */
  MkdededeDecodeStatus mkdedede_ds_decode(const char *password, DsGhostDataC *out_data);

  /**
   * Encodes ghost data into a 16-character Mario Kart: Double Dash!! password
   * @param data Pointer to the struct containing the track info to encode
   * @param out_password Pointer to a char array of at least 17 bytes where the password will be written (null-terminated)
   * @return MKDEDEDE_SUCCESS on success, or an error status if parameters are invalid
   */
  MkdededeDecodeStatus mkdedede_mkdd_encode(const MkddGhostDataC *data, char *out_password);

  /**
   * Encodes ghost data into a 16-character Mario Kart DS password
   * @param data Pointer to the struct containing the track info to encode
   * @param out_password Pointer to a char array of at least 17 bytes where the password will be written (null-terminated)
   * @return MKDEDEDE_SUCCESS on success, or an error status if parameters are invalid
   */
  MkdededeDecodeStatus mkdedede_ds_encode(const DsGhostDataC *data, char *out_password);

/**
 * Get the display name for a Mario Kart: Double Dash!! course
 * @param course_index The course index (0-15)
 * @return Pointer to a static string with the course name, or NULL if invalid
 */
const char *mkdedede_mkdd_course_name(uint8_t course_index);

/**
 * Get the display name for a Mario Kart: Double Dash!! character
 * @param character_index The character index (0-19)
 * @return Pointer to a static string with the character name, or NULL if invalid
 */
const char *mkdedede_mkdd_character_name(uint8_t character_index);

/**
 * Get the display name for a Mario Kart: Double Dash!! kart
 * @param kart_index The kart index (0-20)
 * @return Pointer to a static string with the kart name, or NULL if invalid
 */
const char *mkdedede_mkdd_kart_name(uint8_t kart_index);

/**
 * Get the number of laps for a Mario Kart: Double Dash!! course
 * @param course_index The course index (0-15)
 * @return The lap count, or 0 if invalid
 */
uint32_t mkdedede_mkdd_course_laps(uint8_t course_index);

/**
 * Format a Mario Kart: Double Dash!! lap time as a string
 * @param time_ms The time in milliseconds
 * @param out_str Pointer to a char array of at least 16 bytes for output
 * @return MKDEDEDE_SUCCESS on success, or MKDEDEDE_NULL_POINTER if out_str is NULL
 */
MkdededeDecodeStatus mkdedede_mkdd_format_time(uint32_t time_ms, char *out_str);

/**
 * Get the display name for a Mario Kart DS course
 * @param course_index The course index (0-31)
 * @return Pointer to a static string with the course name, or NULL if invalid
 */
const char *mkdedede_ds_course_name(uint8_t course_index);

/**
 * Get the display name for a Mario Kart DS character
 * @param character_index The character index (0-12)
 * @return Pointer to a static string with the character name, or NULL if invalid
 */
const char *mkdedede_ds_character_name(uint8_t character_index);

/**
 * Get the display name for a Mario Kart DS kart
 * @param kart_index The kart index (0-36)
 * @return Pointer to a static string with the kart name, or NULL if invalid
 */
const char *mkdedede_ds_kart_name(uint8_t kart_index);

/**
 * Format a Mario Kart DS race time as a string
 * @param time_ms The time in milliseconds
 * @param out_str Pointer to a char array of at least 16 bytes for output
 * @return MKDEDEDE_SUCCESS on success, or MKDEDEDE_NULL_POINTER if out_str is NULL
 */
MkdededeDecodeStatus mkdedede_ds_format_time(uint32_t time_ms, char *out_str);

#ifdef __cplusplus
}
#endif

#endif // MKDEDEDE_H
