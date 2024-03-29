use deku::prelude::*;

#[derive(Debug, DekuRead)]
pub struct Packet {
    pub packet_format: u16,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub packet_id: u8,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_ident: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: u8,
    #[deku(ctx = "*packet_id")]
    pub packet_type: PacketType,
}

#[derive(Debug, DekuRead)]
#[deku(ctx = "packet_id: u8", id = "packet_id")]
pub enum PacketType {
    #[deku(id = "0")]
    Motion(PacketMotionData),

    #[deku(id = "1")]
    Session(PacketSessionData),

    #[deku(id = "2")]
    LapData(PacketLapData),

    #[deku(id = "3")]
    Event(PacketEvent),

    #[deku(id = "4")]
    Participant(PacketParticipantData),

    #[deku(id = "5")]
    CarSetup(PacketCarSetupData),

    #[deku(id = "6")]
    CarTelemetry(PacketCarTelemetryData),

    #[deku(id = "7")]
    CarStatus(PacketCarStatusData),

    #[deku(id = "8")]
    FinalClassification(PacketFinalClassificationData),

    #[deku(id = "9")]
    LobbyInfo(PacketLobbyInfoData),

    #[deku(id = "10")]
    CarDamage(CarDamageData),

    #[deku(id = "11")]
    SessionHistory(SessionHistory),
}

#[derive(Debug, DekuRead)]
pub struct PacketMotionData {
    car_motion_data: [CarMotionData; 22],
    suspension_position: [f32; 4],
    suspension_velocity: [f32; 4],
    suspension_acceleration: [f32; 4],
    wheel_speed: [f32; 4],
    wheel_slip: [f32; 4],
    local_velocity_x: f32,
    local_velocity_y: f32,
    local_velocity_z: f32,
    angular_velocity_x: f32,
    angular_velocity_y: f32,
    angular_velocity_z: f32,
    angular_acceleration_x: f32,
    angular_acceleration_y: f32,
    angular_acceleration_z: f32,
    front_wheel_1s_angle: f32,
}

#[derive(Debug, DekuRead)]
pub struct CarMotionData {
    word_position_x: f32,
    word_position_y: f32,
    word_position_z: f32,
    word_velocity_x: f32,
    word_velocity_y: f32,
    word_velocity_z: f32,
    world_forward_dir_x: i16,
    world_forward_dir_y: i16,
    world_forward_dir_z: i16,
    world_right_dir_x: i16,
    world_right_dir_y: i16,
    world_right_dir_z: i16,
    g_force_lateral: f32,
    g_force_longitudinal: f32,
    g_force_vertical: f32,
    yaw: f32,
    patch: f32,
    roll: f32,
}

#[derive(Debug, DekuRead)]
pub struct MarshallZone {
    zone_start: f32,
    zone_flag: i8, // TODO enum
}

#[derive(Debug, DekuRead)]
pub struct WeatherForecastSample {
    session_type: u8, // TODO enum
    time_offset: u8,
    weather: u8, // TODO enum
    track_temperature: i8,
    air_temperature: i8,
}

#[derive(Debug, DekuRead)]
pub struct PacketSessionData {
    weather: u8, // TODO enum
    track_temperature: i8,
    air_temperature: i8,
    total_laps: u8,
    track_length: u16,
    session_type: u8, // TODO enum
    track_id: i8,     // TODO enum
    formula: u8,      // TODO enum
    session_time_left: u16,
    session_duration: u16,
    pit_speed_limit: u8,
    game_paused: u8,
    is_spectating: u8,
    spectator_car_index: u8,
    sli_pro_native_support: u8, // TODO enum
    num_marshal_zones: u8,
    #[deku(count = "num_marshal_zones")]
    marshal_zones: Vec<MarshallZone>,
    safety_car_status: u8, // TODO enum
    network_game: u8,      // TODO enum
    num_weather_forecast_samples: u8,
    #[deku(count = "num_weather_forecast_samples")]
    weather_forcast_samples: Vec<WeatherForecastSample>,
    forcast_accuracy: u8,
    ai_difficulty: u8,
    season_link_ident: u32,
    weekend_link_ident: u32,
    session_link_ident: u32,
    pitstop_window_ideal_lap: u8,
    pitstop_window_latest_lap: u8,
    pitstop_rejoin_position: u8,
    steering_assist: u8,
    braking_assist: u8,
    grearbox_assist: u8,
    pit_assist: u8,
    pit_release_assist: u8,
    ers_assist: u8,
    drs_assist: u8,
    dynamic_racing_line: u8,
    dynamic_racing_line_type: u8,
    game_mode: u8,
    rule_set: u8,
    time_of_day: u32,
    session_length: u8,
}

#[derive(Debug, DekuRead)]
pub struct LapData {
    pub last_lap_time: u32,
    pub current_lap_time: u32,
    pub sector1_time_inms: u16,
    pub sector2_time_inms: u16,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub safety_car_delta: f32,
    pub car_position: u8,
    pub current_lap_num: u8,
    pub pit_status: u8,
    pub num_pit_stops: u8,
    pub sector: u8,
    pub current_lap_invalid: u8,
    pub penalties: u8,
    pub warnings: u8,
    pub num_unserved_drive_through_pens: u8,
    pub num_unserved_stop_go_pens: u8,
    pub grid_position: u8,
    pub driver_status: u8,
    pub result_status: u8,
    pub pitlane_timer_active: u8,
    pub pitlane_time_in_lane: u16,
    pub pitstop_timer: u16,
    pub pitstop_should_serve_pen: u8,
}

#[derive(Debug, DekuRead)]
pub struct PacketLapData {
    pub lap_data: [LapData; 22],
    time_trail_pb_car_idx: u8,
    time_trail_rival_car_idx: u8,
}

#[derive(Debug, DekuRead)]
#[deku(type = "[u8; 4]")]
pub enum PacketEvent {
    #[deku(id = "[b'S', b'S', b'T', b'A']")]
    SessionStarted,
    #[deku(id = "[b'S', b'E', b'N', b'D']")]
    SessionEnabled,
    #[deku(id = "[b'F', b'T', b'L', b'P']")]
    FastestLap(FastestLap),
    #[deku(id = "[b'R', b'T', b'M', b'T']")]
    Retirement(Retirement),
    #[deku(id = "[b'D', b'R', b'S', b'E']")]
    DRSEnabled,
    #[deku(id = "[b'D', b'R', b'S', b'D']")]
    DRSDisabled,
    #[deku(id = "[b'T', b'M', b'P', b'T']")]
    TeamMateInPits,
    #[deku(id = "[b'C', b'H', b'Q', b'F']")]
    ChequeredFlag,
    #[deku(id = "[b'R', b'C', b'W', b'N']")]
    RaceWinner(RaceWinner),
    #[deku(id = "[b'P', b'E', b'N', b'A']")]
    PenaltyIssued(Penalty),
    #[deku(id = "[b'S', b'P', b'T', b'P']")]
    SpeedTrapTriggered(SpeedTrap),
    #[deku(id = "[b'S', b'T', b'L', b'G']")]
    StartLights(StartLights),
    #[deku(id = "[b'L', b'G', b'O', b'T']")]
    LightsOuts,
    #[deku(id = "[b'D', b'T', b'S', b'V']")]
    DriveThroughServed(DriveThroughServed),
    #[deku(id = "[b'S', b'G', b'S', b'V']")]
    StopGoServed(StopGoServed),
    #[deku(id = "[b'F', b'L', b'B', b'K']")]
    FlashBack(FlashBack),
    #[deku(id = "[b'B', b'U', b'T', b'N']")]
    ButtonStatus(ButtonStatus),
}

#[derive(Debug, DekuRead)]
pub struct FastestLap {
    vehicle_idx: u8,
    lap_time: f32,
}

#[derive(Debug, DekuRead)]
pub struct Retirement {
    vehicle_idx: u8,
}

#[derive(Debug, DekuRead)]
pub struct RaceWinner {
    vehicle_idx: u8,
}

#[derive(Debug, DekuRead)]
pub struct Penalty {
    penalty_type: u8,
    infringment_type: u8,
    vehicle_inx: u8,
    other_vehicle_idx: u8,
    time: u8,
    lap_num: u8,
    places_gained: u8,
}

#[derive(Debug, DekuRead)]
pub struct SpeedTrap {
    vehicle_idx: u8,
    speed: f32,
    is_overall_fastest_in_session: u8,
    is_driver_fatest_in_session: u8,
    fatest_vehicle_idx_in_session: u8,
    fastest_speed_in_session: f32,
}

#[derive(Debug, DekuRead)]
pub struct StartLights {
    num_of_lights: u8,
}

#[derive(Debug, DekuRead)]
pub struct DriveThroughServed {
    vehicle_idx: u8,
}

#[derive(Debug, DekuRead)]
pub struct StopGoServed {
    vehicle_idx: u8,
}

#[derive(Debug, DekuRead)]
pub struct FlashBack {
    frame_identifier: u32,
    session_time: f32,
}

#[derive(Debug, DekuRead)]
pub struct ButtonStatus {
    button_status: u32,
}

#[derive(Debug, DekuRead)]
pub struct ParticipantData {
    ai_controlled: u8, // TODO bool
    driver_id: u8,     // TODO enum?
    network_id: u8,    // TODO enum?
    team_id: u8,       // TODO enum?
    my_team: u8,
    race_number: u8,
    nationality: u8,
    name: [u8; 48],
    your_telemetry: u8, // TODO enum
}

#[derive(Debug, DekuRead)]
pub struct PacketParticipantData {
    num_active_cars: u8,
    #[deku(count = "num_active_cars")]
    participants: Vec<ParticipantData>,
}

#[derive(Debug, DekuRead)]
pub struct CarSetupData {
    front_wing: u8,
    rear_wing: u8,
    on_throttle: u8,
    off_throttle: u8,
    front_camber: f32,
    rear_camber: f32,
    front_toe: f32,
    rear_toe: f32,
    front_suspension: u8,
    rear_suspension: u8,
    front_anti_roll_bar: u8,
    rear_anti_roll_bar: u8,
    front_suspension_height: u8,
    rear_suspension_height: u8,
    break_pressure: u8,
    break_bias: u8,
    rear_left_tyre_pressure: f32,
    rear_right_tyre_pressure: f32,
    front_left_tyre_pressure: f32,
    front_right_tyre_pressure: f32,
    ballast: u8,
    fuel_load: f32,
}

#[derive(Debug, DekuRead)]
pub struct PacketCarSetupData {
    car_setups: [CarSetupData; 22],
}

#[derive(Debug, DekuRead)]
pub struct CarTelemetryData {
    speed: u16,
    throttle: f32,
    steer: f32,
    brake: f32,
    clutch: u8,
    gear: i8, // TODO enum
    engine_rpm: u16,
    drs: u8, // TODO enum
    rev_lights_percent: u8,
    rev_lights_bitvalue: u16,
    brakes_temperature: [u16; 4],
    tyres_surface_temperature: [u8; 4],
    tyres_inner_temperature: [u8; 4],
    engine_temperature: u16,
    tyres_pressure: [f32; 4],
    surface_type: [u8; 4],
}

#[derive(Debug, DekuRead)]
pub struct PacketCarTelemetryData {
    car_telemetry_data: [CarTelemetryData; 22],
    mfd_panel_index: u8,
    mfd_panel_index_secondary_player: u8,
    suggested_gear: u8, // TODO enum
}

#[derive(Debug, DekuRead)]
pub struct CarStatusData {
    traction_control: u8,
    anti_lock_brakes: u8,
    fuel_mix: u8,
    front_brake_bias: u8,
    pit_limiter_status: u8,
    fuel_in_tank: f32,
    fuel_capacity: f32,
    fuel_remaining_laps: f32,
    max_rpm: u16,
    idle_rpm: u16,
    max_gears: u8,
    drs_allowed: u8,
    drs_activation_distance: u16,
    actual_tyre_compound: u8, // TODO enum
    visual_tyre_compound: u8, // TODO enum
    tyres_age_laps: u8,
    vehicle_fia_flags: i8,
    ers_store_energy: f32,
    ers_deploy_mode: u8,
    ers_harvested_this_lap_mguk: f32,
    ers_harvested_this_lap_mguh: f32,
    ers_deployted_this_lap: f32,
    network_paused: u8,
}

#[derive(Debug, DekuRead)]
pub struct PacketCarStatusData {
    car_status_data: [CarStatusData; 22],
}

#[derive(Debug, DekuRead)]
pub struct FinalClassificationData {
    position: u8,
    num_laps: u8,
    grid_position: u8,
    points: u8,
    num_pit_stops: u8,
    result_status: u8, // TODO enum
    best_lap_time: f32,
    total_race_time: f64,
    penalties_time: u8,
    num_penalties: u8,
    num_tyre_stints: u8,
    tyre_stints_actual: [u8; 8],
    tyre_stints_visual: [u8; 8],
    tyre_stints_end_laps: [u8; 8],
}

#[derive(Debug, DekuRead)]
pub struct PacketFinalClassificationData {
    num_cars: u8,
    #[deku(count = "num_cars")]
    classificatin_data: Vec<FinalClassificationData>,
}

#[derive(Debug, DekuRead)]
pub struct LobbyInfoData {
    ai_controlled: u8,
    team_id: u8,
    nationality: u8,
    name: [u8; 48],
    ready_status: u8, // TODO enum
}

#[derive(Debug, DekuRead)]
pub struct PacketLobbyInfoData {
    num_players: u8,
    lobby_players: [LobbyInfoData; 22],
}

#[derive(Debug, DekuRead)]
pub struct CarDamageData {
    car_damage: [CarDamage; 22],
}

#[derive(Debug, DekuRead)]
pub struct CarDamage {
    tyres_wear: [f32; 4],
    tyres_damage: [u8; 4],
    brakes_damage: [u8; 4],
    front_left_wing_damage: u8,
    front_right_wing_damage: u8,
    rear_wing_damage: u8,
    floor_damage: u8,
    diffuser_damage: u8,
    sidepod_damage: u8,
    drs_fault: u8,
    ers_fault: u8,
    gearbox_damage: u8,
    engine_damage: u8,
    engine_mguh_wear: u8,
    engine_es_wear: u8,
    engine_ce_wear: u8,
    engine_ice_wear: u8,
    engine_mguk_wear: u8,
    engine_tc_wear: u8,
    engine_blown: u8,
    engine_seized: u8,
}

#[derive(Debug, DekuRead)]
pub struct SessionHistory {
    car_idx: u8,
    num_laps: u8,
    num_tyre_stints: u8,
    best_laptime_lapnum: u8,
    best_sector1_lapnum: u8,
    best_sector2_lapnum: u8,
    best_sector3_lapnum: u8,
    lap_history: [LapHistory; 100],
    tyre_stint_history: [TyreStintHistory; 8],
}

#[derive(Debug, DekuRead)]
pub struct LapHistory {
    lap_time_in_ms: u32,
    sector_one_time_in_ms: u16,
    sector_two_time_in_ms: u16,
    sector_three_time_in_ms: u16,
    lapt_valid_big_flags: u8,
}

#[derive(Debug, DekuRead)]
pub struct TyreStintHistory {
    endlap: u8,
    tyre_actual_compound: u8,
    typre_visual_compound: u8,
}
