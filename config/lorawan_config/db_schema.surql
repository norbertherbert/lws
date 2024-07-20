

----------------------------------------------------
-- NS SESSION Data (primary key: DevAddr)
----------------------------------------------------

-- session_context:                            # data stored in the NS's session context 
--     cipher_id:           DEFAULT
--     version_id:          1.0
--     nwk_s_key:           '00112233445566778899aabbccddeeff'
--     ## f_nwk_s_int_key:  '00112233445566778899aabbccddeeff'
--     ## s_nwk_s_int_key:  '00112233445566778899aabbccddeeff'
--     ## nwk_s_enc_key:    '00112233445566778899aabbccddeeff'
--     app_s_key:           '00112233445566778899aabbccddeeff'
--     dev_addr:            0x00000001
--     f_cnt_up:            0x00000000
--     f_cnt_down:          0x00000000         # LW1.0.4 line #678
--     ## n_f_cnt_down:     0x00000000         # LW1.2.0 D45 line #908
--     ## a_f_cnt_down:     0x00000000
--     ## rj_count_02:      0x0000

DEFINE TABLE ns_session SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD id              ON ns_session TYPE record<int> PERMISSIONS FULL;
-- DEFINE FIELD cipher_id       ON ns_session TYPE string PERMISSIONS FULL;
-- DEFINE FIELD version_id      ON ns_session TYPE string PERMISSIONS FULL;
DEFINE FIELD nwk_s_key       ON ns_session TYPE string PERMISSIONS FULL;
-- DEFINE FIELD f_nwk_s_int_key ON ns_session TYPE string PERMISSIONS FULL;
-- DEFINE FIELD s_nwk_s_int_key ON ns_session TYPE string PERMISSIONS FULL;
-- DEFINE FIELD nwk_s_enc_key   ON ns_session TYPE string PERMISSIONS FULL;
DEFINE FIELD app_s_key       ON ns_session TYPE string PERMISSIONS FULL;
DEFINE FIELD f_cnt_up        ON ns_session TYPE int PERMISSIONS FULL;
DEFINE FIELD f_cnt_down      ON ns_session TYPE int PERMISSIONS FULL;
-- DEFINE FIELD n_f_cnt_down    ON ns_session TYPE int PERMISSIONS FULL;
-- DEFINE FIELD a_f_cnt_down    ON ns_session TYPE int PERMISSIONS FULL;
-- DEFINE FIELD rj_count_02     ON ns_session TYPE int PERMISSIONS FULL;
DEFINE FIELD x_device        ON ns_session TYPE record(ns_device) PERMISSIONS FULL;

----------------------------------------------------
-- NS DEVICE Data (primary key: DevEUI)
----------------------------------------------------

-- ns:                                           # Parameters stored on the Network Server
--     # parameters with 'x_' prefix are proprietary
--     x_is_enabled:          true
--     x_dev_addr:            0x00000001         # if emty, then it should be assigned automatically from a pool
--     x_activation_type:     OTAInternalJS      # OTAInternalJS or OTA or ABP
--     x_join_eui_white_list:                    # if empty, all JoinEUIs are accepted
--       - 0xaabbccddaabbccdd
--       - 0xaabbccddaabbccee
--     device_profile_id:     LW1.2_EU868_ClassA # devvice_profiles.yaml
--     service_profile_id:    GoldService        # service_profiles.yaml
--     routing_profile_id:    My1stWebhook       # routing_profiles.yaml

DEFINE TABLE ns_device SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD id                 ON ns_device TYPE int PERMISSIONS FULL;
DEFINE FIELD x_is_enabled       ON ns_device TYPE bool PERMISSIONS FULL;
DEFINE FIELD x_lora_wan_version ON ns_device TYPE string PERMISSIONS FULL;
DEFINE FIELD x_session          ON ns_device TYPE option<record(ns_session)> PERMISSIONS FULL;


----------------------------------------------------
-- JS DEVICE Data (primary key: DevEUI)
----------------------------------------------------

-- js:                                           # Parameters stored on the Join Server
--     app_key:               '00112233445566778899aabbccddeeff'
--     ## nwk_key:            '00112233445566778899aabbccddeeff'
--     ## js_int_key:         '00112233445566778899aabbccddeeff'
--     ## js_enc_key:         '00112233445566778899aabbccddeeff'
--     home_net_id:           0xaabbcc
--     as_id:                 ???
--     ## a way to select the preferred network
--     lora_wan_version:      '1.0.4'            # It is part of the device profile too, must be consistent!

--     dev_nonce:             0x0000             # LW1.2.0 D45 line #2368
--     ## rj_count_1:         0x0000
--     join_nonce:            0x000000

DEFINE TABLE js_device SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD id               ON js_device TYPE int PERMISSIONS FULL;
DEFINE FIELD app_key          ON js_device TYPE string PERMISSIONS FULL;
-- DEFINE FIELD nwk_key    ON js_device TYPE string PERMISSIONS FULL;
-- DEFINE FIELD js_int_key ON js_device TYPE string PERMISSIONS FULL;
-- DEFINE FIELD js_enc_key ON js_device TYPE string PERMISSIONS FULL;
DEFINE FIELD home_net_id      ON js_device TYPE int PERMISSIONS FULL;
-- DEFINE FIELD as_id            ON js_device TYPE int PERMISSIONS FULL;
-- DEFINE FIELD "a field that defines a way to select the preferred network"
DEFINE FIELD lora_wan_version ON js_device TYPE string PERMISSIONS FULL;
DEFINE FIELD dev_nonce        ON js_device TYPE int PERMISSIONS FULL;
-- DEFINE FIELD rj_count_1 ON js_device TYPE int PERMISSIONS FULL;
DEFINE FIELD join_nonce       ON js_device TYPE int PERMISSIONS FULL;





UPDATE ns_device:1229951823519665356 CONTENT {"x_is_enabled":true,"x_lora_wan_version":"1.0.4"};

UPDATE ns_session:2864434397 CONTENT {"nwk_s_key":"01020304050607080102030405060708", "app_s_key":"01020304050607080102030405060708","f_cnt_up":1,"f_cnt_down":1,"x_device":ns_device:1229951823519665356};
-- add "x_session_id":2864434397 to ns_device:1229951823519665356
UPDATE ns_device:1229951823519665356 SET x_session = ns_session:2864434397;

UPDATE js_device:1229951823519665356 CONTENT {"app_key":"01020304050607080102030405060708","home_net_id":2,"lora_wan_version":"1.0.4","dev_nonce":1234,"join_nonce":1234};