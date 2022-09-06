pub struct Packet {
    id: u16,
    data: vec<u8>
}


// Packet IDs taken from
// [here](https://github.com/Itsyuka/osu-packet/blob/master/src/packets.js)
enum PacketIDs {
    CLIENT_SEND_USER_STATUS,
    CLIENT_SEND_IRC_MESSAGE,
    CLIENT_EXIT,
    CLIENT_REQUEST_STATUS_UPDATE,
    CLIENT_PONG,
    BANCHO_LOGIN_REPLY,
    BANCHO_COMMAND_ERROR,
    BANCHO_SEND_MESSAGE,
    BANCHO_PING,
    BANCHO_HANDLE_IRC_CHANGE_USERNAME,
    BANCHO_HANDLE_IRC_QUIT,
    BANCHO_HANDLE_OSU_UPDATE,
    BANCHO_HANDLE_USER_QUIT,
    BANCHO_SPECTATOR_JOINED,
    BANCHO_SPECTATOR_LEFT,
    BANCHO_SPECTATE_FRAMES,
    CLIENT_START_SPECTATING,
    CLIENT_STOP_SPECTATING,
    CLIENT_SPECTATE_FRAMES,
    BANCHO_VERSION_UPDATE,
    CLIENT_ERROR_REPORT,
    CLIENT_CANT_SPECTATE,
    BANCHO_SPECTATOR_CANT_SPECTATE,
    BANCHO_GET_ATTENTION,
    BANCHO_ANNOUNCE,
    CLIENT_SEND_IRC_MESSAGE_PRIVATE,
    BANCHO_MATCH_UPDATE,
    BANCHO_MATCH_NEW,
    BANCHO_MATCH_DISBAND,
    CLIENT_LOBBY_PART,
    CLIENT_LOBBY_JOIN,
    CLIENT_MATCH_CREATE,
    CLIENT_MATCH_JOIN,
    CLIENT_MATCH_PART,
    BANCHO_MATCH_JOIN_SUCCESS,
    BANCHO_MATCH_JOIN_FAIL,
    CLIENT_MATCH_CHANGE_SLOT,
    CLIENT_MATCH_READY,
    CLIENT_MATCH_LOCK,
    CLIENT_MATCH_CHANGE_SETTINGS,
    BANCHO_FELLOW_SPECTATOR_JOINED,
    BANCHO_FELLOW_SPECTATOR_LEFT,
    CLIENT_MATCH_START,
    BANCHO_MATCH_START,
    CLIENT_MATCH_SCORE_UPDATE,
    BANCHO_MATCH_SCORE_UPDATE,
    CLIENT_MATCH_COMPLETE,
    BANCHO_MATCH_TRANSFER_HOST,
    CLIENT_MATCH_CHANGE_MODS,
    CLIENT_MATCH_LOAD_COMPLETE,
    BANCHO_MATCH_ALL_PLAYERS_LOADED,
    CLIENT_MATCH_NO_BEATMAP,
    CLIENT_MATCH_NOT_READY,
    CLIENT_MATCH_FAILED,
    BANCHO_MATCH_PLAYER_FAILED,
    BANCHO_MATCH_COMPLETE,
    CLIENT_MATCH_HAS_BEATMAP,
    CLIENT_MATCH_SKIP_REQUEST,
    BANCHO_MATCH_SKIP,
    BANCHO_UNAUTHORISED,
    CLIENT_CHANNEL_JOIN,
    BANCHO_CHANNEL_JOIN_SUCCESS,
    BANCHO_CHANNEL_AVAILABLE,
    BANCHO_CHANNEL_REVOKED ,
    BANCHO_CHANNEL_AVAILABLE_AUTOJOIN,
    CLIENT_BEATMAP_INFO_REQUEST,
    BANCHO_BEATMAP_INFO_REPLY,
    CLIENT_MATCH_TRANSFER_HOST,
    BANCHO_LOGIN_PERMISSIONS,
    BANCHO_FRIENDS_LIST,
    CLIENT_FRIEND_ADD,
    CLIENT_FRIEND_REMOVE,
    BANCHO_PROTOCOL_NEGOTIATION,
    BANCHO_TITLE_UPDATE,
    CLIENT_MATCH_CHANGE_TEAM,
    CLIENT_CHANNEL_LEAVE,
    CLIENT_RECEIVE_UPDATES,
    BANCHO_MONITOR,
    BANCHO_MATCH_PLAYER_SKIPPED,
    CLIENT_SET_IRC_AWAY_MESSAGE,
    BANCHO_USER_PRESENCE,
    CLIENT_USER_STATS_REQUEST,
    BANCHO_RESTART,
    CLIENT_INVITE,
    BANCHO_INVITE,
    BANCHO_CHANNEL_LISTING_COMPLETE,
    CLIENT_MATCH_CHANGE_PASSWORD,
    BANCHO_MATCH_CHANGE_PASSWORD,
    BANCHO_BAN_INFO,
    CLIENT_SPECIAL_MATCH_INFO_REQUEST,
    BANCHO_USER_SILENCED,
    BANCHO_USER_PRESENCE_SINGLE,
    BANCHO_USER_PRESENCE_BUNDLE,
    CLIENT_USER_PRESENCE_REQUEST,
    CLIENT_USER_PRESENCE_REQUEST_ALL,
    CLIENT_USER_TOGGLE_BLOCK_NON_FRIEND_PM,
    BANCHO_USER_PM_BLOCKED,
    BANCHO_TARGET_IS_SILENCED,
    BANCHO_VERSION_UPDATE_FORCED,
    BANCHO_SWITCH_SERVER,
    BANCHO_ACCOUNT_RESTRICTED,
    BANCHO_RTX,
    CLIENT_MATCH_ABORT,
    BANCHO_SWITCH_TOURNEY_SERVER,
    CLIENT_SPECIAL_JOIN_MATCH_CHANNEL, 
    CLIENT_SPECIAL_LEAVE_MATCH_CHANNEL,
    }