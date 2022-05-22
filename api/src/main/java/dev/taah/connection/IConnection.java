package dev.taah.connection;

import java.util.UUID;

/**
 * @author Taah
 * @project crewmate
 * @since 4:49 PM [20-05-2022]
 */
public interface IConnection<P>
{
    UUID getUuid();

    String getClientName();

    void sendPacket(P packet);
}
