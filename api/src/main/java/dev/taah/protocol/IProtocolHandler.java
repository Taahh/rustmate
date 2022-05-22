package dev.taah.protocol;

import dev.taah.connection.IConnection;

import java.util.Map;

/**
 * @author Taah
 * @project crewmate
 * @since 4:51 PM [20-05-2022]
 */
public interface IProtocolHandler<P, R, C extends IConnection<?>>
{
    void registerPacket(int id, Class<? extends P> clazz);

    P getPacket(int id);

    R getReliablePacket(int id);
    Map<Integer, Class<? extends P>> getPackets();
}
