package dev.taah;

import com.google.common.collect.Maps;
import dev.taah.handler.PacketHandler;
import dev.taah.protocol.ProtocolHandler;
import dev.taah.util.UDPServerChannel;
import io.netty.bootstrap.Bootstrap;
import io.netty.bootstrap.ServerBootstrap;
import io.netty.buffer.ByteBuf;
import io.netty.buffer.ByteBufUtil;
import io.netty.channel.*;
import io.netty.channel.nio.NioEventLoopGroup;
import io.netty.channel.socket.nio.NioDatagramChannel;
import io.netty.handler.timeout.ReadTimeoutHandler;
import org.jetbrains.annotations.Nullable;

import java.net.InetSocketAddress;
import java.util.Map;
import java.util.Optional;

public class CrewmateServer
{
    public static final Map<InetSocketAddress, ChannelHandlerContext> CONNECTIONS = Maps.newHashMap();
	public static final ProtocolHandler HANDLER = new ProtocolHandler();

    public static void main(String[] args)
    {
        ServerBootstrap bootstrap = new ServerBootstrap()
		.group(new DefaultEventLoopGroup())
				.childHandler(new PacketHandler())
		.childHandler(new ChannelInitializer<Channel>() {
			@Override
			protected void initChannel(Channel channel) {
				channel.pipeline()
				.addLast(new ReadTimeoutHandler(2))
						.addLast(new PacketHandler())
						.addLast(new ChannelDuplexHandler(){
							@Override
							public void write(ChannelHandlerContext ctx, Object msg, ChannelPromise promise) throws Exception
							{
								super.write(ctx, msg, promise);
							}
						});
			}
		});
		if (args.length > 0) {
			int ioThreads = Integer.parseInt(args[0]);
			bootstrap.channelFactory(() -> new UDPServerChannel(ioThreads));
		} else {
			bootstrap.channel(UDPServerChannel.class);
		}
		bootstrap.bind(22023).syncUninterruptibly();
		System.out.println("Started server");
    }

    public static ChannelHandlerContext getOrCreate(InetSocketAddress address, @Nullable ChannelHandlerContext context)
    {
        if (!CONNECTIONS.containsKey(address))
        {
            return CONNECTIONS.put(address, context);
        }
        return CONNECTIONS.get(address);
    }
}
