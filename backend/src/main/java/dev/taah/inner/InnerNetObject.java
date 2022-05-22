package dev.taah.inner;

import dev.taah.packet.IDeserializable;
import dev.taah.packet.ISerializable;
import dev.taah.util.HazelMessage;
import lombok.Data;

/**
 * @author Taah
 * @project crewmate
 * @since 8:32 PM [20-05-2022]
 */
@Data
public abstract class InnerNetObject implements ISerializable, IDeserializable
{
    private final int netId;

}
