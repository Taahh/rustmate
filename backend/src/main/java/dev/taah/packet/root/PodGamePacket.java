package dev.taah.packet.root;

import dev.taah.connection.PlayerConnection;
import dev.taah.packet.ReliablePacket;
import dev.taah.util.GameCode;
import dev.taah.util.PacketBuffer;

/**
 * @author Taah
 * @project crewmate
 * @since 6:42 PM [20-05-2022]
 */
public class PodGamePacket extends ReliablePacket<PodGamePacket>
{
    private String podType;

    @Override
    public void deserialize(PacketBuffer buffer)
    {
        System.out.println("Active Pod Type: " + buffer.readString());
    }

    @Override
    public void processPacket(PodGamePacket packet, PlayerConnection connection)
    {
    }
}
