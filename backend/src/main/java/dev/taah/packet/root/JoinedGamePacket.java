package dev.taah.packet.root;

import dev.taah.connection.PlayerConnection;
import dev.taah.packet.ReliablePacket;
import dev.taah.player.InnerPlayer;
import dev.taah.server.GameRoom;
import dev.taah.util.GameCode;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;
import lombok.Getter;
import lombok.Setter;

/**
 * @author Taah
 * @project crewmate
 * @since 6:42 PM [20-05-2022]
 */
public class JoinedGamePacket extends ReliablePacket<JoinedGamePacket>
{
    @Getter
    @Setter
    private GameCode gameCode;

    @Getter
    @Setter
    private InnerPlayer joining;

    @Getter
    @Setter
    private GameRoom gameRoom;

    @Override
    public void serialize(PacketBuffer buffer)
    {
        buffer.writeShort(this.getNonce());
        HazelMessage message = HazelMessage.start(0x07);
        message.getPayload().writeInt32(this.gameCode.getGameId());
        message.getPayload().writeInt32(this.joining.getId());
        message.getPayload().writeInt32(this.gameRoom.getPlayers().get(this.gameRoom.getHostUuid()).getId());
        if (this.gameRoom.getHostUuid().equals(this.joining.getUuid()))
        {
            message.getPayload().writePackedInt32(0);
        } else
        {
            System.out.println("OTHER PLAYER COUNT: " + (this.gameRoom.getPlayers().size() - 1));
            message.getPayload().writePackedInt32(this.gameRoom.getPlayers().size() - 1);
            this.gameRoom.getPlayers().values().stream().filter(innerPlayer -> !innerPlayer.getUuid().equals(joining.getUuid())).forEach(innerPlayer ->
            {
                message.getPayload().writePackedInt32(innerPlayer.getId());
                message.getPayload().writeString(innerPlayer.getConnection().getClientName());
                innerPlayer.getConnection().getPlatformData().serialize(message.getPayload());
                message.getPayload().writePackedUInt32(0);
                message.getPayload().writeString("none");
                message.getPayload().writeString("none");
            });
        }
        message.endMessage();
        System.out.println("Joined Message Length: " + message.getLength());
        buffer.writeBytes(message.getPayload().getByteArray());
    }

    @Override
    public void processPacket(JoinedGamePacket packet, PlayerConnection connection)
    {
    }
}
