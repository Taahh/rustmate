package dev.taah.packet.root;

import dev.taah.connection.PlayerConnection;
import dev.taah.packet.DisconnectPacket;
import dev.taah.packet.ReliablePacket;
import dev.taah.player.InnerPlayer;
import dev.taah.server.GameRoom;
import dev.taah.server.GameRoomManager;
import dev.taah.util.GameCode;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;

import java.time.LocalDateTime;
import java.time.Period;
import java.time.temporal.ChronoUnit;
import java.util.concurrent.TimeUnit;

/**
 * @author Taah
 * @project crewmate
 * @since 6:42 PM [20-05-2022]
 */
public class GetGameListPacket extends ReliablePacket<GetGameListPacket>
{
    private static final byte[] ADDRESS_BITS = "127.0.0.1".getBytes();

    @Override
    public void deserialize(PacketBuffer buffer)
    {
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {
        HazelMessage hazelMessage = HazelMessage.start(0x10);
        {
            HazelMessage counts = HazelMessage.start(0x01);
            counts.getPayload().writeUInt32(1);
            counts.getPayload().writeUInt32(1);
            counts.getPayload().writeUInt32(1);
            counts.endMessage();
            hazelMessage.getPayload().writeBytes(counts.getPayload().getByteArray());
        }
        {
            GameRoomManager.getRooms().forEach(gameRoom -> {
                InnerPlayer host = gameRoom.getPlayers().get(gameRoom.getHostUuid());
                HazelMessage gameList = HazelMessage.start(0x00);
                gameList.getPayload().writeBytes(ADDRESS_BITS);
                gameList.getPayload().writeUInt16(22023);
                gameList.getPayload().writeInt32(gameRoom.getGameCode().getGameId());
                gameList.getPayload().writeString(host.getConnection().getClientName());
                gameList.getPayload().writeByte(gameRoom.getPlayers().size());
                gameList.getPayload().writePackedUInt32(ChronoUnit.SECONDS.between(LocalDateTime.now(), gameRoom.getDateTime()));
                gameList.getPayload().writeByte(gameRoom.getGameOptionsData().getMaps());
                gameList.getPayload().writeByte(gameRoom.getGameOptionsData().getImposterCount());
                gameList.getPayload().writeByte(gameRoom.getGameOptionsData().getMaxPlayers());
                gameList.getPayload().writeByte(host.getConnection().getPlatformData().platform());
                gameList.getPayload().writeString(host.getConnection().getPlatformData().platformName());
                gameList.endMessage();
                hazelMessage.getPayload().writeBytes(gameList.getPayload().getByteArray());
            });
        }
        hazelMessage.endMessage();
        buffer.writeBytes(hazelMessage.getPayload().getByteArray());
    }

    @Override
    public void processPacket(GetGameListPacket packet, PlayerConnection connection)
    {
        connection.sendPacket(packet);
    }
}
