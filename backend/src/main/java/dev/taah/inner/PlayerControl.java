package dev.taah.inner;

import dev.taah.data.PlayerOutfit;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;

/**
 * @author Taah
 * @project crewmate
 * @since 11:49 AM [22-05-2022]
 */
public class PlayerControl extends InnerNetObject
{
    public PlayerControl()
    {
        super(3);
    }

    @Override
    public void deserialize(PacketBuffer buffer)
    {
        while (buffer.readerIndex() < buffer.readableBytes())
        {
            HazelMessage message = HazelMessage.read(buffer);
            System.out.println("Player ID: " + message.getTag());
            int outfits = message.getPayload().readByte();
            System.out.println("Outfit Count: " + outfits);
            for (int i = 0; i < outfits; i++)
            {
                System.out.println("Outfit Type: " + message.getPayload().readByte());
                PlayerOutfit outfit = PlayerOutfit.deserialize(message.getPayload());
            }
        }
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {

    }
}
