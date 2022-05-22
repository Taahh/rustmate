package dev.taah.packet.root.gamedata;

import dev.taah.packet.IDeserializable;
import dev.taah.packet.ISerializable;
import dev.taah.util.PacketBuffer;
import lombok.Data;

/**
 * @author Taah
 * @project crewmate
 * @since 9:03 PM [20-05-2022]
 */
@Data
public abstract class AbstractGameData implements ISerializable, IDeserializable
{
    private final int dataId;
}
