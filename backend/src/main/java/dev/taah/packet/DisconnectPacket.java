package dev.taah.packet;

import dev.taah.connection.PlayerConnection;
import dev.taah.server.GameRoom;
import dev.taah.server.GameRoomManager;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;
import io.netty.buffer.ByteBufUtil;
import lombok.Getter;
import lombok.Setter;

/**
 * @author Taah
 * @project crewmate
 * @since 4:59 PM [20-05-2022]
 */
public class DisconnectPacket extends AbstractPacket<DisconnectPacket>
{
    @Getter
    @Setter
    private String reason;

    public DisconnectPacket()
    {
        super(0x09);
    }

    public DisconnectPacket(String reason)
    {
        super(0x09);
        this.reason = reason;
    }

    @Override
    public void deserialize(PacketBuffer buffer)
    {
    }

    @Override
    public void processPacket(DisconnectPacket packet, PlayerConnection connection)
    {
        GameRoom gameRoom = GameRoomManager.getGameRoom(connection.getUuid());
        if (gameRoom != null)
        {
            gameRoom.getPlayers().remove(connection.getUuid());
            if (gameRoom.getPlayers().isEmpty())
            {
                System.out.println("Removed game room");
                GameRoomManager.removeGameRoom(gameRoom);
            } else {
                System.out.println("Assigned new host");
                gameRoom.setHostUuid(gameRoom.getPlayers().get(gameRoom.getPlayers().keySet().iterator().next()).getUuid());
            }
        }

        connection.getChannel().attr(PlayerConnection.CONNECTION_STRING).set(null);
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {
        buffer.writeByte(0x01);
//        System.out.println("Disconnect 1:" + ByteBufUtil.prettyHexDump(buffer));
        HazelMessage message = HazelMessage.start(0x00);
        message.getPayload().writeByte(0x08);
        message.getPayload().writeString(reason);
        message.endMessage();
        buffer.writeBytes(message.getPayload().getByteArray());
    }
}
