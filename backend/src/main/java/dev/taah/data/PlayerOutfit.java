package dev.taah.data;

import dev.taah.packet.IDeserializable;
import dev.taah.packet.ISerializable;
import dev.taah.util.PacketBuffer;
import lombok.Data;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.Setter;

/**
 * @author Taah
 * @project crewmate
 * @since 7:09 PM [20-05-2022]
 */
@Data
public class PlayerOutfit implements ISerializable
{
    private final String playerName;


    public static PlayerOutfit deserialize(PacketBuffer buffer)
    {
        String name = buffer.readString();
        System.out.println("Player Outfit --- Player Name: " + name);
        System.out.println("Color Id: " + buffer.readPackedInt32());
        System.out.println("Hat Id: " + buffer.readString());
        System.out.println("Pet Id: " + buffer.readString());
        System.out.println("Skin Id: " + buffer.readString());
        System.out.println("Visor Id: " + buffer.readString());
        System.out.println("Name Plate Id: " + buffer.readString());
        return new PlayerOutfit(name);
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {

    }
}
