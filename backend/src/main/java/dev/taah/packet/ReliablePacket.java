package dev.taah.packet;

import dev.taah.connection.PlayerConnection;
import dev.taah.packet.root.*;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;
import lombok.Getter;
import lombok.SneakyThrows;

import java.util.HashMap;
import java.util.Map;

/**
 * @author Taah
 * @project crewmate
 * @since 6:19 PM [20-05-2022]
 */
public class ReliablePacket<T extends ReliablePacket<T>> extends AbstractPacket<T>
{
    private final Map<Integer, Class<? extends ReliablePacket<?>>> rootPackets = new HashMap<>();
    @Getter
    private HazelMessage hazelMessage;

    public ReliablePacket()
    {
        super(0x01);
        this.registerRootPacket(0x00, (Class<? extends T>) HostGamePacket.class);
        this.registerRootPacket(0x01, (Class<? extends T>) JoinGamePacket.class);

        this.registerRootPacket(0x15, (Class<? extends T>) PodGamePacket.class);
        this.registerRootPacket(0x05, (Class<? extends T>) GameDataPacket.class);
        this.registerRootPacket(0x10, (Class<? extends T>) GetGameListPacket.class);
    }

    @Override
    public void deserialize(PacketBuffer buffer)
    {
        this.hazelMessage = HazelMessage.read(buffer);
        System.out.printf("New Reliable Packet with length %s and tag %s%n", hazelMessage.getLength(), hazelMessage.getTag());
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {

    }

    public void registerRootPacket(int id, Class<? extends T> clazz)
    {
        rootPackets.put(id, clazz);
    }

    @SneakyThrows
    public ReliablePacket<?> getRootPacket(int id)
    {
        Class<? extends ReliablePacket<?>> root = rootPackets.get(id);
        if (root == null)
        {
            return null;
        }
        return (ReliablePacket<?>) root.getConstructors()[0].newInstance();
    }

    @Override
    public void processPacket(T packet, PlayerConnection connection)
    {
        connection.sendPacket(new AcknowledgePacket(packet.getNonce()));
        T other = (T) getRootPacket(packet.getHazelMessage().getTag());
        if (other != null)
        {
            other.setNonce(packet.getNonce());
            other.deserialize(this.hazelMessage.getPayload());
            other.processPacket(other, connection);
        }
    }
}
