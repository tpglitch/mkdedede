#ifndef MKDEDEDE_H
#define MKDEDEDE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct MkddGhostDataC {
    uint8_t course;
    uint8_t kart;
    uint8_t driver1;
    uint8_t driver2;
    uint32_t total_time_ms;
    uint32_t best_lap_ms;
} MkddGhostDataC;

typedef struct DsGhostDataC {
    uint8_t course;
    uint8_t character;
    uint8_t kart;
    uint32_t time_ms;
    uint16_t player_name[2];
} DsGhostDataC;

typedef enum MkdededeDecodeStatus {
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
MkdededeDecodeStatus mkdedede_mkdd_decode(const char* password, MkddGhostDataC* out_data);

/**
 * Decodes a Mario Kart DS 16-character password into ghost data
 * @param password Null-terminated string containing the password to decode
 * @param out_data Pointer to the struct where decoded track info will be written
 * @return MKDEDEDE_SUCCESS on success, or an error status on failure
 */
MkdededeDecodeStatus mkdedede_ds_decode(const char* password, DsGhostDataC* out_data);

/**
 * Encodes ghost data into a 16-character Mario Kart: Double Dash!! password
 * @param data Pointer to the struct containing the track info to encode
 * @param out_password Pointer to a char array of at least 17 bytes where the password will be written (null-terminated)
 * @return MKDEDEDE_SUCCESS on success, or an error status if parameters are invalid
 */
MkdededeDecodeStatus mkdedede_mkdd_encode(const MkddGhostDataC* data, char* out_password);

/**
 * Encodes ghost data into a 16-character Mario Kart DS password
 * @param data Pointer to the struct containing the track info to encode
 * @param out_password Pointer to a char array of at least 17 bytes where the password will be written (null-terminated)
 * @return MKDEDEDE_SUCCESS on success, or an error status if parameters are invalid
 */
MkdededeDecodeStatus mkdedede_ds_encode(const DsGhostDataC* data, char* out_password);

#ifdef __cplusplus
}
#endif

#endif // MKDEDEDE_H
