package dev.taah.protocol;

import com.google.common.collect.Maps;
import dev.taah.connection.PlayerConnection;
import dev.taah.packet.*;
import lombok.SneakyThrows;

import java.util.Map;

/**
 * @author Taah
 * @project crewmate
 * @since 4:52 PM [20-05-2022]
 */
public class ProtocolHandler implements IProtocolHandler<AbstractPacket, ReliablePacket<?>, PlayerConnection>
{
    private static final Map<Integer, Class<? extends AbstractPacket>> PACKET_MAP = Maps.newHashMap();

    public ProtocolHandler()
    {
        this.registerPacket(0x08, HelloPacket.class);
        this.registerPacket(0x01, ReliablePacket.class);
        this.registerPacket(0x0c, PingPacket.class);
//        this.registerPacket(0x0a, AcknowledgePacket.class);
        this.registerPacket(0x09, DisconnectPacket.class);
    }


    @Override
    public void registerPacket(int id, Class<? extends AbstractPacket> clazz)
    {
        PACKET_MAP.put(id,  clazz);
    }

    @SneakyThrows
    @Override
    public AbstractPacket getPacket(int id)
    {
        Class<? extends AbstractPacket> packet = PACKET_MAP.get(id);
        if (packet == null) {
            return null;
        }
        return (AbstractPacket<?>) packet.getConstructors()[0].newInstance();
    }

    @SneakyThrows
    @Override
    public ReliablePacket<?> getReliablePacket(int id)
    {
        Class<? extends AbstractPacket> packet = PACKET_MAP.get(id);
        if (packet == null) {
            return null;
        }
        ReliablePacket<?> reliablePacket = (ReliablePacket<?>) packet.getConstructors()[0].newInstance();
        return reliablePacket.getRootPacket(id);
    }

    @Override
    public Map<Integer, Class<? extends AbstractPacket>> getPackets()
    {
        return PACKET_MAP;
    }
}
