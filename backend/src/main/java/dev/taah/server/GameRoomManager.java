package dev.taah.server;

import com.google.common.collect.ImmutableList;
import com.google.common.collect.Maps;
import dev.taah.util.GameCode;

import java.util.Map;
import java.util.Objects;
import java.util.UUID;

/**
 * @author Taah
 * @project crewmate
 * @since 11:41 AM [21-05-2022]
 */
public class GameRoomManager
{
    private static final Map<GameCode, GameRoom> ROOMS = Maps.newHashMap();

    public static void addGameRoom(GameRoom room) {
        ROOMS.put(room.getGameCode(), room);
    }

    public static boolean gameRoomExists(GameCode code) {
        return ROOMS.containsKey(code);
    }

    public static void removeGameRoom(GameCode code) {
        ROOMS.remove(code);
    }

    public static void removeGameRoom(GameRoom room) {
        removeGameRoom(room.getGameCode());
    }

    public static void updateRoom(GameRoom gameRoom) {
        ROOMS.replace(gameRoom.getGameCode(), gameRoom);
    }

    public static GameRoom getGameRoom(GameCode gameCode) {
        for (GameCode code : ROOMS.keySet()) {
            if (code.equals(gameCode)) {
                return ROOMS.get(code);
            }
        }
        return null;
    }

    public static GameRoom getGameRoom(UUID uuid) {
        return ROOMS.values().stream().filter(gameRoom -> gameRoom.getPlayers().containsKey(uuid) || gameRoom.getHostUuid().equals(uuid)).findFirst().orElse(null);
    }

    public static ImmutableList<GameRoom> getRooms()
    {
        return ImmutableList.copyOf(ROOMS.values());
    }
}
