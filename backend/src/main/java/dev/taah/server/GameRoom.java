package dev.taah.server;

import com.google.common.collect.Maps;
import dev.taah.data.GameOptionsData;
import dev.taah.packet.AbstractPacket;
import dev.taah.player.InnerPlayer;
import dev.taah.util.GameCode;
import lombok.Data;

import java.time.LocalDateTime;
import java.util.Arrays;
import java.util.Map;
import java.util.UUID;

/**
 * @author Taah
 * @project crewmate
 * @since 11:05 AM [21-05-2022]
 */
@Data
public class GameRoom
{
    private final LocalDateTime dateTime = LocalDateTime.now();
    private final Map<UUID, InnerPlayer> players = Maps.newHashMap();
    private final GameCode gameCode;

    private GameOptionsData gameOptionsData;
    private UUID hostUuid;

    public void sendPacket(AbstractPacket<?> packet, UUID... ignore)
    {
        if (ignore != null)
        {
            players.values().stream().map(InnerPlayer::getConnection).filter(connection -> Arrays.stream(ignore).noneMatch(uuid -> uuid.equals(connection.getUuid()))).forEach(connection ->
            {
                connection.sendPacket(packet);
            });
            return;
        }
        players.values().stream().map(InnerPlayer::getConnection).forEach(connection ->
        {
            connection.sendPacket(packet);
        });
    }
}
