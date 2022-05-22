package dev.taah.packet;

import dev.taah.util.PacketBuffer;

/**
 * @author Taah
 * @project crewmate
 * @since 4:48 PM [20-05-2022]
 */
public interface IDeserializable
{
    void deserialize(PacketBuffer buffer);
}
