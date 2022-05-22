package dev.taah.packet.root.gamedata;

import dev.taah.inner.InnerNetObject;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;
import lombok.SneakyThrows;

import java.util.HashMap;
import java.util.Map;

/**
 * @author Taah
 * @project crewmate
 * @since 9:04 PM [20-05-2022]
 */
public class SpawnGameData extends AbstractGameData
{
    private static final Map<Integer, Class<? extends InnerNetObject>> INNER_NET_OBJECTS = new HashMap<>();

    public SpawnGameData()
    {
        super(0x04);
//        this.registerInnerNetObject(0x04, SpawnGameData.class);
    }

    public void registerInnerNetObject(int id, Class<? extends InnerNetObject> clazz)
    {
        INNER_NET_OBJECTS.put(id, clazz);
    }

    @SneakyThrows
    public InnerNetObject getById(int id) {
        Class<? extends InnerNetObject> innerNetObject = INNER_NET_OBJECTS.get(id);
        if (innerNetObject == null)
        {
            return null;
        }
        return (InnerNetObject) innerNetObject.getConstructors()[0].newInstance();
    }

    @Override
    public void deserialize(PacketBuffer buffer)
    {
        System.out.println("Net Obj Spawn ID: " + buffer.readPackedUInt32());
        System.out.println("Owner ID: " + buffer.readPackedUInt32());
        System.out.println("Flags: " + buffer.readByte());
        int components = (int) buffer.readPackedUInt32();
        for (int i = 0; i < components; i++)
        {
            InnerNetObject object = new InnerNetObject((int) buffer.readPackedUInt32(), HazelMessage.read(buffer));
            System.out.println("Start Of Object: " + object.getData().getTag());
            System.out.println("Inner Object ID: " + object.getNetId());
        }
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {

    }
}
