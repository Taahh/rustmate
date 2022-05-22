package dev.taah.player;

import dev.taah.connection.PlayerConnection;
import dev.taah.server.GameRoom;
import lombok.Data;

import java.util.UUID;

/**
 * @author Taah
 * @project crewmate
 * @since 11:06 AM [21-05-2022]
 */
@Data
public class InnerPlayer
{
    private final UUID uuid;
    private final PlayerConnection connection;
    private final GameRoom gameRoom;

    private int id;
}
