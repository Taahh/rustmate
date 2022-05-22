package dev.taah.packet;

import dev.taah.connection.PlayerConnection;
import dev.taah.util.PacketBuffer;

/**
 * @author Taah
 * @project crewmate
 * @since 5:32 PM [20-05-2022]
 */
public class AcknowledgePacket extends AbstractPacket<AcknowledgePacket>
{
    public AcknowledgePacket()
    {
        this(-1);
    }

    public AcknowledgePacket(int nonce)
    {
        super(0x0a, nonce);
    }

    @Override
    public void deserialize(PacketBuffer buffer)
    {

    }

    @Override
    public void processPacket(AcknowledgePacket packet, PlayerConnection connection)
    {
        System.out.println("Sending ack packet: " + packet.getNonce());
        connection.sendPacket(new AcknowledgePacket(packet.getNonce()));
    }

    @Override
    public int getPacketId()
    {
        return 0x0a;
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {
        buffer.writeShort(this.getNonce());
        buffer.writeByte(0xff);
    }
}
