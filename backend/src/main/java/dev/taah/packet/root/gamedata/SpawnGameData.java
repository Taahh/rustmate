package dev.taah.packet.root.gamedata;

import dev.taah.inner.GameData;
import dev.taah.inner.InnerNetObject;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;
import io.netty.buffer.ByteBufUtil;
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
    private static final Map<Long, Class<? extends InnerNetObject>> INNER_NET_OBJECTS = new HashMap<>();

    public SpawnGameData()
    {
        super(0x04);
//        this.registerInnerNetObject(0x04, SpawnGameData.class);
        this.registerInnerNetObject(0x01, GameData.class);
    }

    public void registerInnerNetObject(long id, Class<? extends InnerNetObject> clazz)
    {
        INNER_NET_OBJECTS.put(id, clazz);
    }

    @SneakyThrows
    public InnerNetObject getById(long id) {
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
        System.out.println("Owner ID: " + buffer.readPackedInt32());
        System.out.println("Flags: " + buffer.readByte());
        int components = (int) buffer.readPackedInt32();
        System.out.println("Components: " + components);
        for (int i = 0; i < components; i++)
        {
//            InnerNetObject object = new InnerNetObject((int) buffer.readPackedUInt32(), HazelMessage.read(buffer));
//            System.out.println("Start Of Object: " + object.getData().getTag());
//            System.out.println("Inner Object ID: " + object.getNetId());
//            InnerNetObject object = getById()
            long netId = buffer.readPackedUInt32();
            InnerNetObject object = getById(netId);
            System.out.println("Remaining buffer: " + ByteBufUtil.prettyHexDump(buffer));
            HazelMessage msg = HazelMessage.read(buffer);
            System.out.println("Starting Tag: " + msg.getTag());
            System.out.println("Starting Length: " + msg.getLength());
            if (object != null && msg != null)
            {
                System.out.println("Deserializing component: " + object.getClass().getSimpleName());
                object.deserialize(new PacketBuffer(msg.getPayload().slice(0, msg.getLength())));
            }
        }
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {

    }
}
