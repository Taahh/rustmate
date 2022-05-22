package dev.taah.util;

import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class HazelMessage
{
    private int length;
    private int tag;
    private PacketBuffer payload;

    public static HazelMessage read(PacketBuffer buffer)
    {
        HazelMessage message = new HazelMessage();
        try
        {
            message.setLength(buffer.readUInt16());
            message.setTag(buffer.readUnsignedByte());
        } catch (IndexOutOfBoundsException e) {
            return null;
        }
        message.setPayload(buffer);
        return message;
    }

    public static HazelMessage start(PacketBuffer buffer, int tag)
    {
        HazelMessage hazelMessage = new HazelMessage();
        hazelMessage.setTag(tag);
        hazelMessage.setPayload(buffer);
        buffer.writeUInt16(0x00);
        buffer.writeByte(tag);
        return hazelMessage;
    }

    public static HazelMessage start(int tag)
    {
        return start(new PacketBuffer(), tag);
    }

    public void endMessage()
    {
        int length = this.getPayload().readableBytes() - 3;
        this.getPayload().markWriterIndex();
        this.getPayload().writerIndex(0);
        this.getPayload().writeUInt16(length);
        this.setLength(length);
        this.getPayload().resetWriterIndex();
    }
}
