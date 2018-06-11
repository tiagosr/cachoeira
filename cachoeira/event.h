#pragma once
#include "types.h"
#include "3rdparty/simplesignal.h"

namespace cc {
    class BaseEvent {
    public:
        typedef size_t Family;
        virtual ~BaseEvent() {}
    protected:
        inline static Family family_counter_ = 0;
    };

    using EventSignal = Simple::Signal<void(const void*)>;
    using EventSignalPtr = shared_ptr<EventSignal>;
    using EventSignalWeakPtr = weak_ptr<EventSignal>;

    template <typename Derived>
    class Event : public BaseEvent {
    public:
        static Family family() {
            static Family family = family_counter_++;
            return family;
        }
    };

    class BaseReceiver {
    public:
        virtual ~BaseReceiver() {
            for (auto connection : m_connections) {
                auto& ptr = connection.second.first;
                if (!ptr.expired()) {
                    ptr.lock()->disconnect(connection.second.second);
                }
            }
        }

        size_t connected_signals() const {
            size_t size = 0;
            for (auto connection : m_connections) {
                if (!connection.second.first.expired()) {
                    size++;
                }
            }
            return size;
        }


    private:
        friend class EventManager;
        unordered_map<BaseEvent::Family, pair<EventSignalWeakPtr, size_t>> m_connections;
    };

    template <typename Derived>
    class Receiver : public BaseReceiver {
    public:
        virtual ~Receiver() {}
    };

    class EventManager : public non_copyable {
    public:
        EventManager() {}
        virtual ~EventManager() {}

        template <typename E, typename Receiver>
        void subscribe(Receiver &receiver) {
            void (Receiver::*receive)(const E&) = &Receiver::receive;
            auto sig = signal_for(Event<E>::family());
            auto wrapper = EventCallbackWrapper<E>(std::bind(receive, &receiver, std::placeholders::_1));
            auto connection = sig->connect(wrapper);
            BaseReceiver &base = receiver;
            base.m_connections.insert(make_pair(Event<E>::family(), make_pair(EventSignalWeakPtr(sig), connection)));
        }

        template <typename E, typename Receiver>
        void unsubscribe(Receiver &receiver) {
            BaseReceiver &base = receiver;
            assert(base.m_connections.find(Event<E>::family()) != base.m_connections.end());
            auto connection_pair = base.m_connections[Event<E>::family()];
            auto connection = connection_pair.second;
            auto &ptr = connection_pair.first;
            if (!ptr.expired()) {
                ptr.lock()->disconnect(connection);
            }
            base.m_connections.erase(Event<E>::family());
        }

        template <typename E>
        void emit(const E &ev) {
            auto sig = signal_for(Event<E>::family());
            sig->emit(&ev);
        }

        template <typename E>
        void emit(unique_ptr<E> ev) {
            auto sig = signal_for(Event<E>::family());
            sig->emit(ev.get())
        }

        template <typename E, typename... Args>
        void emit(Args &&... args) {
            E ev = E(std::forward<Args> args...);
            auto sig = signal_for(Event<E>::family());
            sig->emit(&ev);
        }

        size_t connected_receivers() const {
            size_t size = 0;
            for (EventSignalPtr handler : m_handlers) {
                if (handler) size += handler->size();
            }
            return size;
        }

    private:
        EventSignalPtr & signal_for(size_t id) {
            if (id >= m_handlers.size())
                m_handlers.resize(id + 1);
            if (!m_handlers[id])
                m_handlers[id] = make_shared<EventSignal>();
            return m_handlers[id];
        }

        template <typename E>
        struct EventCallbackWrapper {
            explicit EventCallbackWrapper(std::function<void (const E&)> callback): callback(callback) {}
            void operator()(const void *event) { callback(*(static_cast<const E*>(event))); }
            std::function<void(const E&)> callback;
        };

        vector<EventSignalPtr> m_handlers;
    };
}
