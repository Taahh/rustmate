package dev.taah.packet;

import dev.taah.connection.PlayerConnection;
import dev.taah.util.PacketBuffer;

import static java.lang.Math.floor;

/**
 * @author Taah
 * @project crewmate
 * @since 4:59 PM [20-05-2022]
 */
public class PingPacket extends AbstractPacket<PingPacket>
{

    public PingPacket()
    {
        super(0x08);
    }

    @Override
    public void deserialize(PacketBuffer buffer)
    {
    }

    @Override
    public void processPacket(PingPacket packet, PlayerConnection connection)
    {
        connection.sendPacket(new AcknowledgePacket(packet.getNonce()));
    }

    @Override
    public int getPacketId()
    {
        return 1;
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {

    }
}
