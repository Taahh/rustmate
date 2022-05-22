package dev.taah.data;

import dev.taah.packet.IDeserializable;
import dev.taah.packet.ISerializable;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;
import io.netty.buffer.ByteBufUtil;
import lombok.Data;

/**
 * @author Taah
 * @project crewmate
 * @since 3:31 PM [21-05-2022]
 */
public record PlatformData(byte platform, String platformName) implements ISerializable
{
    @Override
    public void serialize(PacketBuffer buffer)
    {
        HazelMessage hazelMessage = HazelMessage.start(platform);
        hazelMessage.getPayload().writeString(platformName);
        hazelMessage.endMessage();
        System.out.println("Platform Data Serialized: " + ByteBufUtil.prettyHexDump(hazelMessage.getPayload()));
        buffer.writeBytes(hazelMessage.getPayload().getByteArray());
    }
}
