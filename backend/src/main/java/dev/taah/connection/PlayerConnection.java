package dev.taah.connection;

import dev.taah.data.PlatformData;
import dev.taah.packet.AbstractPacket;
import dev.taah.packet.AcknowledgePacket;
import dev.taah.util.PacketBuffer;
import io.netty.buffer.ByteBufUtil;
import io.netty.channel.Channel;
import io.netty.util.AttributeKey;
import lombok.Data;
import lombok.Getter;
import lombok.Setter;

import java.util.UUID;

/**
 * @author Taah
 * @project crewmate
 * @since 5:00 PM [20-05-2022]
 */
@Data
public class PlayerConnection implements IConnection<AbstractPacket<?>>
{
    public static final AttributeKey<PlayerConnection> CONNECTION_STRING = AttributeKey.newInstance("player_conn");

    private final Channel channel;
    private final UUID uuid;
    private String clientName;

    private PlatformData platformData;

    private int nonce = 0;
    @Override
    public UUID getUuid()
    {
        return uuid;
    }

    @Override
    public String getClientName()
    {
        return this.clientName;
    }

    @Override
    public void sendPacket(AbstractPacket<?> packet)
    {
        if (!(packet instanceof AcknowledgePacket))
        {
            packet.setNonce(nonce+=1);
        }
        PacketBuffer buffer = new PacketBuffer();
        buffer.writeByte(packet.getPacketId());
        buffer.writeShort(packet.getNonce());
        packet.serialize(buffer);
        System.out.printf("Sending %s packet with buffer %n%s", packet.getClass().getSimpleName(), ByteBufUtil.prettyHexDump(buffer));
        this.channel.writeAndFlush(buffer);
    }
}
