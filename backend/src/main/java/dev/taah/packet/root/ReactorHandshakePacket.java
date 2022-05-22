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
public class ReactorHandshakePacket extends ReliablePacket<ReactorHandshakePacket>
{
    @Override
    public void serialize(PacketBuffer buffer)
    {
        HazelMessage message = HazelMessage.start(255);
        message.getPayload().writeByte(0x00);
        message.getPayload().writeString("Among Us");
        message.getPayload().writeString("0.1.1");
        message.getPayload().writePackedUInt32(0);
        message.endMessage();
        System.out.println("Reactor Handshake Packet: " + message.getLength());
        buffer.writeBytes(message.getPayload().getByteArray());
    }

    @Override
    public void processPacket(ReactorHandshakePacket packet, PlayerConnection connection)
    {
    }
}
